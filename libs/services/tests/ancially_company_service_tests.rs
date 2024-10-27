#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::AncillaryCompanyService;
    use services::prelude::*;

    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_ancillary_company_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /ancillary_companies/{id}/ endpoint
        let company_id = 140756665106487;
        let ancillary_company = get_mock_ancillary_company(company_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/ancillary_companies/{}/", company_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&ancillary_company).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = AncillaryCompanyService::new(&client);

        // Call the method under test
        let result = service.get(company_id).await;

        // Assert the result
        assert!(result.is_ok());
        let company = result.unwrap();
        assert_eq!(company.id, company_id);
        assert_eq!(company.name, "Test Ancillary Company");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_ancillary_companies_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let company_type = AncillaryCompanyType::Imaging;
        let mock_company = get_mock_ancillary_company(140756665106487);

        let companies = vec![mock_company.clone()];
        let companies_json = serde_json::to_string(&companies).unwrap();

        // Mock the GET /ancillary_companies/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/ancillary_companies/")
                .query_param("type", company_type.to_string().to_lowercase());

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {companies_json}, \"next\": null, \"previous\": null, \"count\": 1 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = AncillaryCompanyService::new(&client);

        // Prepare query parameters
        let query_params = AncillaryCompanyQueryParams {
            name: None,
            company_type: Some(company_type),
        };

        // Call the method under test
        let result = service.find(query_params).await;

        // Assert the result
        assert!(result.is_ok());
        let companies = result.unwrap().results;
        assert_eq!(companies.len(), 1);
        assert_eq!(companies[0].id, 140756665106487);
        assert_eq!(companies[0].name, "Test Ancillary Company");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_get_ancillary_company_not_found() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /ancillary_companies/{id}/ endpoint to return 404
        let company_id = 999999;
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/ancillary_companies/{}/", company_id));
            then.status(404)
                .header("Content-Type", "application/json")
                .body(
                    r#"{
                        "detail": "Not found."
                    }"#,
                );
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = AncillaryCompanyService::new(&client);

        // Call the method under test
        let result = service.get(company_id).await;

        // Assert the result
        assert!(result.is_err());
        let error = result.err().unwrap();

        // Match the error variant
        match error {
            Error::ClientError(client_error) => match client_error {
                client::Error::NotFound(_) => {
                    // Expected error
                }
                _ => panic!("Expected NotFound error"),
            },
            _ => panic!("Expected ClientError wrapping NotFound error"),
        }

        // Ensure the mock was called
        mock.assert_async().await;
    }

    fn get_mock_ancillary_company(company_id: i64) -> AncillaryCompany {
        AncillaryCompany {
            id: company_id,
            name: "Test Ancillary Company".to_string(),
        }
    }
}
