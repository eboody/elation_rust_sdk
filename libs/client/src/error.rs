use std::num::ParseFloatError;

use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

/// A specialized `Result` type for the SDK.
///
/// This is defined as a convenience so that you don't have to write out
/// `core::result::Result<T, Error>` every time.
///
/// # Example
///
/// ```
/// use your_crate::Result;
///
/// fn your_function() -> Result<()> {
///     // Your code here...
///     Ok(())
/// }
/// ```
pub type Result<T> = core::result::Result<T, Error>;

/// The error type for the SDK.
///
/// This enum represents all possible errors that can occur when using the SDK.
/// It includes variants for common HTTP errors, as well as errors originating
/// from external crates like `reqwest` and `url`.
#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
    /// An error originating from the `reqwest` library.
    ///
    /// This variant wraps `reqwest::Error`, which can occur during HTTP requests.
    #[from]
    ReqwestError(#[serde_as(as = "DisplayFromStr")] reqwest::Error),

    /// An error related to invalid HTTP header values.
    ///
    /// This variant wraps `reqwest::header::InvalidHeaderValue`, which occurs
    /// when constructing headers with invalid values.
    #[from]
    HeaderValueError(#[serde_as(as = "DisplayFromStr")] reqwest::header::InvalidHeaderValue),

    /// An error that occurs when parsing a URL fails.
    ///
    /// This variant wraps `url::ParseError`, which can occur when constructing URLs.
    #[from]
    ParseUrl(#[serde_as(as = "DisplayFromStr")] url::ParseError),

    /// Indicates that a required request body was missing.
    ///
    /// This error occurs when a request expected a body but none was provided.
    BodyMissing,

    /// Represents an API-specific error returned by the server.
    ///
    /// Contains a message describing the error details.
    ApiError(String),

    /// An error that occurs when parsing a float from a string fails.
    ///
    /// This variant wraps `std::num::ParseFloatError`.
    ParseFloat(#[serde_as(as = "DisplayFromStr")] ParseFloatError),

    /// Bad Request (HTTP 400).
    ///
    /// The server could not understand the request due to invalid syntax.
    BadRequest(String), // 400

    /// Unauthorized (HTTP 401).
    ///
    /// Authentication is required and has failed or has not yet been provided.
    Unauthorized(String), // 401

    /// Forbidden (HTTP 403).
    ///
    /// The client does not have access rights to the content.
    Forbidden(String), // 403

    /// Not Found (HTTP 404).
    ///
    /// The server can not find the requested resource.
    NotFound(String), // 404

    /// Conflict (HTTP 409).
    ///
    /// The request conflicts with the current state of the server.
    Conflict(String), // 409

    /// Unsupported Media Type (HTTP 415).
    ///
    /// The media format of the requested data is not supported by the server.
    UnsupportedMediaType(String), // 415

    /// Too Many Requests (HTTP 429).
    ///
    /// The client has sent too many requests in a given amount of time.
    TooManyRequests(String), // 429

    /// Internal Server Error (HTTP 500).
    ///
    /// The server encountered an unexpected condition that prevented it from fulfilling the request.
    InternalServerError(String), // 500

    /// Service Unavailable (HTTP 503).
    ///
    /// The server is not ready to handle the request, often due to maintenance or overload.
    ServiceUnavailable(String), // 503

    /// Gateway Timeout (HTTP 504).
    ///
    /// The server was acting as a gateway and did not receive a timely response from the upstream server.
    GatewayTimeout(String), // 504
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    /// Formats the error using the `Display` trait.
    ///
    /// This implementation delegates to the `Debug` representation,
    /// which includes the error variant name and any associated data.
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate::Error;
    ///
    /// let error = Error::NotFound("Resource not found".into());
    /// println!("{}", error);
    /// // Output: NotFound("Resource not found")
    /// ```
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<ParseFloatError> for Error {
    /// Converts a `ParseFloatError` into an `Error::ParseFloat` variant.
    ///
    /// This allows automatic conversion of `ParseFloatError` into `Error` using the `?` operator.
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate::Error;
    /// use std::num::ParseFloatError;
    ///
    /// fn parse_value(s: &str) -> Result<f64, Error> {
    ///     let value: f64 = s.parse()?;
    ///     Ok(value)
    /// }
    /// ```
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloat(value)
    }
}
// endregion: --- Error Boilerplate
