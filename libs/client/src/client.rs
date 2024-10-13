use std::fmt::Debug;

use config::elation_config;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Response, StatusCode,
};
use serde_with::serde_as;
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use url::Url;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub use crate::{Error, Result};

use crate::config::{self};

/// Trait for query parameter types
pub trait Params: Serialize + Default + Debug {}

/// Blanket implementation for any type that implements `Serialize` and `Default`.
impl<T: Serialize + Default + Debug> Params for T {}

/// Represents a paginated response from the Elation EMR API.
///
/// The `PaginatedResponse` struct is a generic container for handling paginated data returned by the API.
/// It includes metadata about the total number of items, links to navigate between pages, and the actual results.
///
/// # Type Parameters
///
/// * `T` - The type of the items contained in the `results` list.
///
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The total number of items available.
    ///
    /// This field represents the total count of items that match the request criteria,
    /// regardless of pagination.
    pub count: usize,

    /// The URL to the next page of results, if any.
    ///
    /// This field contains the full URL to retrieve the next page of results.
    /// It is `None` if there are no more pages.
    pub next: Option<String>,

    /// The URL to the previous page of results, if any.
    ///
    /// This field contains the full URL to retrieve the previous page of results.
    /// It is `None` if you are on the first page.
    pub previous: Option<String>,

    /// The list of results for the current page.
    ///
    /// This is a vector containing items of type `T`, representing the data returned for the current page.
    pub results: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    /// Checks if there is a next page of results.
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// Checks if there is a previous page of results.
    pub fn has_previous(&self) -> bool {
        self.previous.is_some()
    }
}

impl<T> PaginatedResponse<T>
where
    T: DeserializeOwned,
{
    /// Fetches the next page of results, if available.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the `Client` instance to make the request.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no next page or if the request fails.
    pub async fn fetch_next_page(&self, client: &Client) -> Result<Option<PaginatedResponse<T>>> {
        if let Some(next_url) = &self.next {
            let response = client.get_full_url(next_url).await?;
            let paginated_response = response.json::<PaginatedResponse<T>>().await?;
            Ok(Some(paginated_response))
        } else {
            Ok(None)
        }
    }
}

/// A client for interacting with the Elation EMR API.
///
/// The `Client` struct provides methods to perform HTTP requests
/// to the Elation EMR API, including GET, POST, PUT, PATCH, and DELETE operations.
/// It handles authentication by obtaining an access token from a token service
/// and sets up default headers for requests.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

/// Enum representing the supported HTTP methods.
///
/// `HttpMethod` is used internally by the `Client` to determine
/// which HTTP method to use for a given request.
#[derive(Serialize, EnumString, Display)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Client {
    /// Creates a new instance of `Client`.
    ///
    /// This function initializes the client by obtaining an access token
    /// from the token service and setting up default headers.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to obtain an access token or
    /// if the `reqwest::Client` cannot be built.
    pub async fn new() -> Result<Self> {
        let access_token = Self::get_access_token().await?;

        let client = Self::create_client(&access_token)?;

        Ok(Self { client })
    }

    /// Retrieves an access token from the token service.
    ///
    /// This function uses an exponential backoff retry strategy to handle transient errors.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to obtain a response from the token service
    /// or if it cannot read the access token from the response.
    async fn get_access_token() -> Result<String> {
        let retry_strategy = ExponentialBackoff::from_millis(10).take(3);

        let base_url = if std::env::var("TEST_ENV").is_ok() {
            return Ok("12345".to_owned());
        } else {
            // Use the real API URL
            Url::parse(&elation_config().TOKEN_SERVICE_URL)?
        };

        let response = Retry::spawn(retry_strategy, || async {
            let request = reqwest::Client::new().get(format!("{}token", base_url));

            request.send().await
        })
        .await?;

        let access_token = response.text().await?;

        Ok(access_token)
    }

    /// Creates a new `reqwest::Client` with default headers.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to build the `reqwest::Client`.
    fn create_client(access_token: &str) -> Result<reqwest::Client> {
        let headers = Self::default_headers(access_token)?;
        Ok(reqwest::Client::builder()
            .default_headers(headers)
            .build()?)
    }

    /// Constructs the default headers for the HTTP client.
    ///
    /// Sets the `Content-Type` and `Accept` headers to `application/json`
    /// and includes the `Authorization` header with the provided API key.
    ///
    /// # Errors
    ///
    /// Returns an error if the `Authorization` header cannot be constructed.
    fn default_headers(api_key: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("accept", HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
        Ok(headers)
    }

    /// Sends a GET request to the specified endpoint with optional query parameters.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    /// * `params` - A struct implementing `Params` that represents the query parameters for the request.
    ///
    /// # Type Parameters
    ///
    /// * `P` - A type that implements the `Params` trait (i.e., `Serialize` and `Default`).
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get<P: Params>(&self, endpoint: &str, params: P) -> Result<Response> {
        self.send_request(HttpMethod::GET, endpoint, None::<&()>, Some(&params))
            .await
    }

    /// Sends a POST request to the specified endpoint with the provided body.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    /// * `body` - The request body to be serialized as JSON.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the request body, which must implement `Serialize`, `Sized`, and `Debug`.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn post<T: Serialize + Sized + Debug>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response> {
        self.send_request(HttpMethod::POST, endpoint, Some(body), None::<&()>)
            .await
    }

    /// Sends a DELETE request to the specified endpoint with the provided ID.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    /// * `id` - The ID of the resource to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn delete(&self, endpoint: &str) -> Result<Response> {
        self.send_request(
            HttpMethod::DELETE,
            &format!("{}", endpoint),
            None::<&()>,
            None::<&()>,
        )
        .await
    }

    /// Sends a PUT request to the specified endpoint with the provided body.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    /// * `body` - The request body to be serialized as JSON.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the request body, which must implement `Serialize`, `Sized`, and `Debug`.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn put<T: Serialize + Sized + Debug>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response> {
        self.send_request(HttpMethod::PUT, endpoint, Some(body), None::<&()>)
            .await
    }

    /// Sends a PATCH request to the specified endpoint with the provided body.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
    /// * `body` - The request body to be serialized as JSON.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the request body, which must implement `Serialize`, `Sized`, and `Debug`.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn patch<T: Serialize + Sized + Debug>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response> {
        self.send_request(HttpMethod::PATCH, endpoint, Some(body), None::<&()>)
            .await
    }

    /// Sends an HTTP request using the specified method, endpoint, and optional body.
    ///
    /// This method constructs the full URL by joining the base API URL and the endpoint,
    /// sets up the request based on the provided HTTP method, and attaches the body if provided.
    /// It handles the response, logging errors and mapping HTTP status codes to custom errors.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use for the request.
    /// * `endpoint` - The API endpoint to send the request to.
    /// * `body` - An optional reference to the request body to be serialized as JSON.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the request body, which must implement `Serialize`, `Sized`, and `Debug`.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if an error occurs while processing the response.
    async fn send_request<T, P>(
        &self,
        method: HttpMethod,
        endpoint: &str,
        body: Option<&T>,
        params: Option<&P>,
    ) -> Result<Response>
    where
        T: Serialize + Sized + Debug,
        P: Params,
    {
        // Check if we are in a testing environment
        let base_url = if std::env::var("TEST_ENV").is_ok() {
            // Use the mock server URL
            Url::parse(
                &std::env::var("MOCK_SERVER_URL")
                    .unwrap_or_else(|_| "http://localhost:1234".to_string()),
            )?
        } else {
            // Use the real API URL
            Url::parse(&elation_config().ELATION_API_URL)?
        };

        let mut url = base_url.join(endpoint)?;

        let mut request_builder = match method {
            HttpMethod::GET => {
                let query = serde_urlencoded::to_string(&params)?;
                url.set_query(Some(&query));

                self.client.get(url)
            }
            HttpMethod::DELETE => self.client.delete(url),
            HttpMethod::POST => self.client.post(url),
            HttpMethod::PUT => self.client.put(url),
            HttpMethod::PATCH => self.client.patch(url),
        };

        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        let response = request_builder.send().await.map_err(Error::from)?;

        match response.error_for_status_ref() {
            Ok(_) => Ok(response),
            Err(e) => {
                let status = e.status();
                let message = response.text().await?;
                log::error!(
                    "Method: {}\nEndpoint: {}\nMessage: {}\nBody: {:#?}",
                    method,
                    endpoint,
                    message,
                    body
                );
                Err(Self::map_error(status, endpoint, &message, body, e))
            }
        }
    }

    /// Maps HTTP status codes to custom error types.
    ///
    /// This function interprets the HTTP status code from a failed request
    /// and returns a corresponding custom `Error` variant with a detailed message.
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code of the response, if available.
    /// * `endpoint` - The API endpoint that was called.
    /// * `message` - The error message from the response body.
    /// * `body` - An optional reference to the request body that was sent.
    /// * `e` - The original `reqwest::Error`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the request body.
    fn map_error<T: Debug>(
        status: Option<StatusCode>,
        endpoint: &str,
        message: &str,
        body: Option<&T>,
        e: reqwest::Error,
    ) -> Error {
        match status {
            Some(StatusCode::BAD_REQUEST) => Error::BadRequest(format!("Bad Request at '{}': Unable to understand the request. Error: {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::CONFLICT) => Error::Conflict(format!("Conflict at '{}': Elation probably thinks this is a malformed resource. Error: {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::UNAUTHORIZED) => Error::Unauthorized(format!("Unauthorized at '{}': {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::FORBIDDEN) => Error::Forbidden(format!("Forbidden at '{}': Access to the requested resource is denied. Error: {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::NOT_FOUND) => Error::NotFound(format!("Not Found at '{}': The requested resource could not be found. Error: {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::UNSUPPORTED_MEDIA_TYPE) => Error::UnsupportedMediaType(format!("Unsupported Media Type at '{}': Ensure you are using application/json as content type. Error: {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::TOO_MANY_REQUESTS) => Error::TooManyRequests(format!("Too Many Requests at '{}': You have hit the rate limit. Try again later. Error: {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::INTERNAL_SERVER_ERROR) => Error::InternalServerError(format!("Internal Server Error at '{}': Something went wrong on the server side. Error: {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::SERVICE_UNAVAILABLE) => Error::ServiceUnavailable(format!("Service Unavailable at '{}': The server is currently too busy. Try again later. Error: {}. Body: {:?}", endpoint, message, body)),
            Some(StatusCode::GATEWAY_TIMEOUT) => Error::GatewayTimeout(format!("Gateway Timeout at '{}': The request took too long to complete. Error: {}. Body: {:?}", endpoint, message, body)),
            //Some(status) if status.is_client_error() => Error::ClientError(format!(
            //    "Client error at '{}': {}. Body: {:?}",
            //    endpoint, message, body
            //)),
            //Some(status) if status.is_server_error() => Error::ServerError(format!(
            //    "Server error at '{}': {}. Body: {:?}",
            //    endpoint, message, body
            //)),
            _ => Error::ReqwestError(e),
        }
    }
    /// Sends a GET request to a full URL.
    ///
    /// This method allows you to send a GET request to a full URL, which is useful when working with pagination.
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL to send the request to.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get_full_url(&self, url: &str) -> Result<Response> {
        let request = self.client.get(url);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Handles the response, checking for errors and mapping them appropriately.
    async fn handle_response(&self, response: Response) -> Result<Response> {
        match response.error_for_status_ref() {
            Ok(_) => Ok(response),
            Err(e) => {
                let status = e.status();
                let message = response.text().await.unwrap_or_default();
                log::error!("Request failed: {}", message);
                Err(Self::map_error(status, "", &message, None::<&()>, e))
            }
        }
    }
    /// Fetches all pages of a paginated resource.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to fetch.
    ///
    /// # Errors
    ///
    /// Returns an error if any request fails.
    pub async fn get_all_pages<T>(&self, endpoint: &str) -> Result<Vec<T>>
    where
        T: DeserializeOwned + Debug,
    {
        let mut results = Vec::new();
        let mut next_url = Some(format!("{}{}", elation_config().ELATION_API_URL, endpoint));

        while let Some(url) = next_url {
            let response = self.get_full_url(&url).await?;
            let paginated_response: PaginatedResponse<T> = response.json().await?;
            results.extend(paginated_response.results);

            next_url = paginated_response.next;
        }

        Ok(results)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use serde_json::json;
//
//    // Import the necessary dependencies
//    use tokio;
//
//    // Helper function to initialize the client
//    async fn init_client() -> Result<Client> {
//        Client::new().await
//    }
//
//    // Test the GET method
//    #[tokio::test]
//    async fn test_get_request() -> Result<()> {
//        let client = init_client().await?;
//        let endpoint = "/your/get/endpoint"; // Replace with a valid endpoint
//
//        let response = client.get(endpoint, ()).await?;
//        assert!(response.status().is_success(), "GET request failed");
//
//        let json: serde_json::Value = response.json().await?;
//        println!("GET Response: {:#?}", json);
//
//        // Add assertions based on expected response
//        // assert_eq!(json["key"], "expected_value");
//
//        Ok(())
//    }
//
//    // Test the POST method
//    #[tokio::test]
//    async fn test_post_request() -> Result<()> {
//        let client = init_client().await?;
//        let endpoint = "/your/post/endpoint"; // Replace with a valid endpoint
//
//        // Construct the body as per the API requirements
//        let body = json!({
//            "key": "value",
//            // Add other necessary fields
//        });
//
//        let response = client.post(endpoint, &body).await?;
//        assert!(response.status().is_success(), "POST request failed");
//
//        let json: serde_json::Value = response.json().await?;
//        println!("POST Response: {:#?}", json);
//
//        // Add assertions based on expected response
//        // assert_eq!(json["key"], "expected_value");
//
//        Ok(())
//    }
//
//    // Test the PUT method
//    #[tokio::test]
//    async fn test_put_request() -> Result<()> {
//        let client = init_client().await?;
//        let endpoint = "/your/put/endpoint"; // Replace with a valid endpoint
//
//        // Construct the body as per the API requirements
//        let body = json!({
//            "key": "new_value",
//            // Add other necessary fields
//        });
//
//        let response = client.put(endpoint, &body).await?;
//        assert!(response.status().is_success(), "PUT request failed");
//
//        let json: serde_json::Value = response.json().await?;
//        println!("PUT Response: {:#?}", json);
//
//        // Add assertions based on expected response
//
//        Ok(())
//    }
//
//    // Test the PATCH method
//    #[tokio::test]
//    async fn test_patch_request() -> Result<()> {
//        let client = init_client().await?;
//        let endpoint = "/your/patch/endpoint"; // Replace with a valid endpoint
//
//        // Construct the body as per the API requirements
//        let body = json!({
//            "key": "patched_value",
//            // Add other necessary fields
//        });
//
//        let response = client.patch(endpoint, &body).await?;
//        assert!(response.status().is_success(), "PATCH request failed");
//
//        let json: serde_json::Value = response.json().await?;
//        println!("PATCH Response: {:#?}", json);
//
//        // Add assertions based on expected response
//
//        Ok(())
//    }
//
//    // Test the DELETE method
//    #[tokio::test]
//    async fn test_delete_request() -> Result<()> {
//        let client = init_client().await?;
//        let endpoint = "/your/delete/endpoint"; // Replace with a valid endpoint
//        let id = "resource_id"; // Replace with a valid resource ID
//
//        let response = client.delete(endpoint, id.to_string()).await?;
//        assert!(response.status().is_success(), "DELETE request failed");
//
//        println!("DELETE Response Status: {:?}", response.status());
//
//        // Optionally, verify that the resource is actually deleted
//
//        Ok(())
//    }
//
//    // Test obtaining an access token
//    #[tokio::test]
//    async fn test_get_access_token() -> Result<()> {
//        let access_token = Client::get_access_token().await?;
//        assert!(!access_token.is_empty(), "Access token is empty");
//
//        println!("Access Token: {}", access_token);
//
//        Ok(())
//    }
//
//    //// Test client initialization
//    //#[tokio::test]
//    //async fn test_client_initialization() -> Result<()> {
//    //    let client = init_client().await?;
//    //    // You can perform additional checks on the client if necessary
//    //
//    //    Ok(())
//    //}
//}
