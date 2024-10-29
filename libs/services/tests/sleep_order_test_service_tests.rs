#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, POST};
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::SleepOrderTestService;
    use services::prelude::*;

    use serial_test::serial;
    use time::OffsetDateTime;

    #[serial]
    #[tokio::test]
    async fn test_get_sleep_order_test_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /sleep_order_tests/{id}/ endpoint
        let test_id = 140756665106487;
        let sleep_order_test = get_mock_sleep_order_test(test_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/sleep_order_tests/{}/", test_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&sleep_order_test).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderTestService::new(&client);

        // Call the method under test
        let result = service.get(test_id).await;

        // Assert the result
        assert!(result.is_ok());
        let test = result.unwrap();
        assert_eq!(test.id, test_id);
        assert_eq!(test.name, "test");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_sleep_order_tests_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let mock_test = get_mock_sleep_order_test(140756665106487);

        let tests = vec![mock_test.clone()];
        let tests_json = serde_json::to_string(&tests).unwrap();

        // Mock the GET /sleep_order_tests/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/sleep_order_tests/")
                .query_param("name", "test");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                "{{ \"results\": {tests_json}, \"next\": null, \"previous\": null, \"count\": 1 }}"
            ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderTestService::new(&client);

        // Prepare query parameters
        let query_params = SleepOrderTestQueryParams {
            name: Some("test".to_string()),
            ..Default::default()
        };

        // Call the method under test
        let result = service.find(query_params).await;

        // Assert the result
        assert!(result.is_ok());
        let tests = result.unwrap().results;
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].id, 140756665106487);
        assert_eq!(tests[0].name, "test");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_sleep_order_test_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the test data to create
        let test_for_create = SleepOrderTestForCreate {
            code: Some("SOT123".to_string()),
            name: "New Sleep Test".to_string(),
            practice: 140756660256772,
        };

        let created_test = SleepOrderTest {
            id: 140756665106488,
            code: Some("SOT123".to_string()),
            name: "New Sleep Test".to_string(),
            practice: Some(140756660256772),
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        };

        // Mock the POST /sleep_order_tests/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/sleep_order_tests")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&test_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&created_test).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderTestService::new(&client);

        // Call the method under test
        let result = service.post(&test_for_create).await;

        // Assert the result
        assert!(result.is_ok());
        let test = result.unwrap();
        assert_eq!(test.id, 140756665106488);
        assert_eq!(test.name, "New Sleep Test");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_sleep_order_test_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the DELETE /sleep_order_tests/{id}/ endpoint
        let test_id = 140756665106487;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/sleep_order_tests/{}/", test_id));
            then.status(204); // No Content
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderTestService::new(&client);

        // Call the method under test
        let result = service.delete(test_id).await;

        // Assert the result
        assert!(result.is_ok());

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_get_sleep_order_test_not_found() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /sleep_order_tests/{id}/ endpoint to return 404
        let test_id = 999999;
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/sleep_order_tests/{}/", test_id));
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
        let service = SleepOrderTestService::new(&client);

        // Call the method under test
        let result = service.get(test_id).await;

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

    fn get_mock_sleep_order_test(test_id: i64) -> SleepOrderTest {
        SleepOrderTest {
            id: test_id,
            code: None,
            name: "test".to_string(),
            practice: Some(140756660256772),
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }
}
