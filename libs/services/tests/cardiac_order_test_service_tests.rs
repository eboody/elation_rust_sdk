#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{GET, POST};
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::CardiacOrderTestService;
    use services::prelude::*;
    use time::OffsetDateTime;

    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_cardiac_order_test_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /cardiac_order_tests/{id}/ endpoint
        let test_id = 140756665106487;
        let cardiac_order_test = get_mock_cardiac_order_test(test_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/cardiac_order_tests/{}/", test_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&cardiac_order_test).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = CardiacOrderTestService::new(&client);

        // Call the method under test
        let result = service.get(test_id).await;

        println!("result: {:#?}", result);

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
    async fn test_create_cardiac_order_test_success() {
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Create the cardiac order test data for the mock
        let test_id = 140756665106488;

        // Prepare the cardiac order test data to create
        let test_for_create = CardiacOrderTestForCreate {
            code: Some("ABC123".to_string()),
            name: "New Cardiac Test".to_string(),
            practice: 140756660256772,
        };

        // Mock the POST /cardiac_order_tests/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/cardiac_order_tests")
                .header("Content-Type", "application/json")
                .json_body_obj(&test_for_create);
            then.status(201)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&get_mock_cardiac_order_test_with_data(
                        test_id,
                        &test_for_create,
                    ))
                    .unwrap(),
                );
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = CardiacOrderTestService::new(&client);

        // Call the method under test
        let result = service.post(&test_for_create).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let created_test = result.unwrap();
        assert_eq!(created_test.id, test_id);
        assert_eq!(created_test.name, "New Cardiac Test");
        assert_eq!(created_test.code.as_deref(), Some("ABC123"));
        assert_eq!(created_test.practice, Some(140756660256772));

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_cardiac_order_tests_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let test1 = get_mock_cardiac_order_test(140756665106487);
        let mut test2 = get_mock_cardiac_order_test(140756665106488);
        test2.name = "another test".to_string();

        let vec_of_tests = serde_json::to_string(&vec![test1.clone(), test2.clone()]).unwrap();

        // Mock the GET /cardiac_order_tests/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/cardiac_order_tests/")
                .query_param("name", "test");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {vec_of_tests}, \"next\": null, \"previous\": null, \"count\": 2 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = CardiacOrderTestService::new(&client);

        // Prepare query parameters
        let query_params = CardiacOrderTestQueryParams {
            name: Some("test".to_string()),
            practice: None,
        };

        // Call the method under test
        let result = service.find(query_params).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let tests = result.unwrap().results;
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0].name, "test");
        assert_eq!(tests[1].name, "another test");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_get_cardiac_order_test_not_found() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /cardiac_order_tests/{id}/ endpoint to return 404
        let test_id = 999999;
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/cardiac_order_tests/{}/", test_id));
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
        let service = CardiacOrderTestService::new(&client);

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

    // Helper function to create a mock cardiac order test
    fn get_mock_cardiac_order_test(test_id: i64) -> CardiacOrderTest {
        CardiacOrderTest {
            code: None,
            created_date: Some(
                OffsetDateTime::parse(
                    "2021-05-26T03:49:04Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            deleted_date: None,
            id: test_id,
            name: "test".to_string(),
            practice: Some(140756660256772),
        }
    }

    // Helper function to create a mock cardiac order test with provided data
    fn get_mock_cardiac_order_test_with_data(
        test_id: i64,
        data: &CardiacOrderTestForCreate,
    ) -> CardiacOrderTest {
        CardiacOrderTest {
            code: data.code.clone(),
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
            id: test_id,
            name: data.name.clone(),
            practice: Some(data.practice),
        }
    }
}
