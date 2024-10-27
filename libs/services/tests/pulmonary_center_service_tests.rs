#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::PulmonaryCenterService;
    use services::prelude::*;

    use serial_test::serial;
    use time::OffsetDateTime;

    #[serial]
    #[tokio::test]
    async fn test_get_pulmonary_center_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /pulmonary_centers/{id}/ endpoint
        let center_id = 140755855671306;
        let pulmonary_center = get_mock_pulmonary_center(center_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/pulmonary_centers/{}/", center_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&pulmonary_center).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = PulmonaryCenterService::new(&client);

        // Call the method under test
        let result = service.get(center_id).await;

        // Assert the result
        assert!(result.is_ok());
        let center = result.unwrap();
        assert_eq!(center.id, center_id);
        assert_eq!(center.location_name, "Test Pulmonary Center");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_pulmonary_centers_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let mock_center = get_mock_pulmonary_center(140755855671306);

        let centers = vec![mock_center.clone()];
        let centers_json = serde_json::to_string(&centers).unwrap();

        // Mock the GET /pulmonary_centers/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/pulmonary_centers/")
                .query_param("location_name", "Test Pulmonary Center");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {centers_json}, \"next\": null, \"previous\": null, \"count\": 1 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = PulmonaryCenterService::new(&client);

        // Prepare query parameters
        let query_params = PulmonaryCenterQueryParams {
            location_name: Some("Test Pulmonary Center".to_string()),
            ..Default::default()
        };

        // Call the method under test
        let result = service.find(query_params).await;

        // Assert the result
        assert!(result.is_ok());
        let centers = result.unwrap().results;
        assert_eq!(centers.len(), 1);
        assert_eq!(centers[0].id, 140755855671306);
        assert_eq!(centers[0].location_name, "Test Pulmonary Center");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_get_pulmonary_center_not_found() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /pulmonary_centers/{id}/ endpoint to return 404
        let center_id = 999999;
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/pulmonary_centers/{}/", center_id));
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
        let service = PulmonaryCenterService::new(&client);

        // Call the method under test
        let result = service.get(center_id).await;

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

    fn get_mock_pulmonary_center(center_id: i64) -> PulmonaryCenter {
        PulmonaryCenter {
            id: center_id,
            address_line1: "123 Elation St".to_string(),
            address_line2: Some("Suite 4".to_string()),
            city: "San Francisco".to_string(),
            company_name: "Test Ancillary Company".to_string(),
            company: 140755855605768,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
            fax: "444-444-4444".to_string(),
            location_name: "Test Pulmonary Center".to_string(),
            phone: "555-555-5555".to_string(),
            practice: Some(65540),
            state: "CA".to_string(),
            zip: "94103".to_string(),
        }
    }
}
