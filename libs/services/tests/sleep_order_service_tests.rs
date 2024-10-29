#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::{orders::*, Icd10Code};
    use services::orders::SleepOrderService;
    use services::prelude::*;

    use serial_test::serial;
    use time::{Date, Month, OffsetDateTime};

    #[serial]
    #[tokio::test]
    async fn test_get_sleep_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /sleep_orders/{id}/ endpoint
        let order_id = 140756377075740;
        let sleep_order = get_mock_sleep_order(order_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/sleep_orders/{}/", order_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&sleep_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderService::new(&client);

        // Call the method under test
        let result = service.get(order_id).await;

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, order_id);
        assert_eq!(order.clinical_reason, "sickness");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_sleep_orders_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let mock_order = get_mock_sleep_order(140756377075740);

        let orders = vec![mock_order.clone()];
        let orders_json = serde_json::to_string(&orders).unwrap();

        // Mock the GET /sleep_orders/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/sleep_orders/")
                .query_param("patient", "140756664516609");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {orders_json}, \"next\": null, \"previous\": null, \"count\": 1 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderService::new(&client);

        // Prepare query parameters
        let query_params = SleepOrderQueryParams {
            patient: Some(140756664516609),
            ..Default::default()
        };

        // Call the method under test
        let result = service.find(query_params).await;

        println!("{result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let orders = result.unwrap().results;
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].id, 140756377075740);
        assert_eq!(orders[0].clinical_reason, "sickness");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_sleep_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the order data to create
        let order_for_create = SleepOrderForCreate {
            ancillary_company: 140755855605768,
            ccs: Some(vec![131074]),
            chart_date: Date::from_calendar_date(2021, Month::May, 26).unwrap(),
            clinical_reason: "sickness".to_string(),
            confidential: true,
            document_date: Date::from_calendar_date(2021, Month::May, 26).unwrap(),
            follow_up_method: Some("email".to_string()),
            icd10_codes: Some(vec![Icd10Code {
                code: "O09.529".to_string(),
                description: Some(
                    "Supervision of elderly multigravida, unspecified trimester".to_string(),
                ),
            }]),
            patient: 140756664516609,
            practice: 140756660256772,
            prescribing_user: 2032,
            sleep_center: Some(140755855671306),
            tests: vec![SleepOrderTestForOrder {
                id: 140756665106487,
            }],
        };

        let created_order = get_mock_sleep_order(140756377075741);

        // Mock the POST /sleep_orders/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/sleep_orders")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&order_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&created_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderService::new(&client);

        // Call the method under test
        let result = service.post(&order_for_create).await;

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, 140756377075741);
        assert_eq!(order.clinical_reason, "sickness");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_sleep_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the order data to update
        let order_for_update = SleepOrderForUpdate {
            clinical_reason: Some("Updated reason".to_string()),
            ..Default::default()
        };

        let updated_order = get_mock_sleep_order(140756377075740);

        // Mock the PATCH /sleep_orders/{id}/ endpoint
        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/sleep_orders/{}/", updated_order.id))
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&order_for_update).unwrap());
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&updated_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderService::new(&client);

        // Call the method under test
        let result = service.patch(updated_order.id, &order_for_update).await;

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, 140756377075740);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_sleep_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the DELETE /sleep_orders/{id}/ endpoint
        let order_id = 140756377075740;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/sleep_orders/{}/", order_id));
            then.status(204); // No Content
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = SleepOrderService::new(&client);

        // Call the method under test
        let result = service.delete(order_id).await;

        // Assert the result
        assert!(result.is_ok());

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_get_sleep_order_not_found() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /sleep_orders/{id}/ endpoint to return 404
        let order_id = 999999;
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/sleep_orders/{}/", order_id));
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
        let service = SleepOrderService::new(&client);

        // Call the method under test
        let result = service.get(order_id).await;

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

    fn get_mock_sleep_order(order_id: i64) -> SleepOrder {
        SleepOrder {
            id: order_id,
            ancillary_company: 140755855605768,
            ccs: Some(vec![131074]),
            chart_date: Some(OffsetDateTime::now_utc()),
            clinical_reason: "sickness".to_string(),
            confidential: true,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
            document_date: Some(OffsetDateTime::now_utc()),
            follow_up_method: Some("email".to_string()),
            icd10_codes: Some(vec![Icd10Code {
                code: "O09.529".to_string(),
                description: Some(
                    "Supervision of elderly multigravida, unspecified trimester".to_string(),
                ),
            }]),
            patient: 140756664516609,
            practice: 140756660256772,
            prescribing_user: 2032,
            signed_date: Some(OffsetDateTime::now_utc()),
            signed_by: Some(1407563456436),
            resolution: Some(Resolution {
                id: 140754511659009,
                document: 140754512183329,
                resolving_document: Some(140754512183329),
                state: ResolutionState::Outstanding,
                note: None,
                created_date: Some(OffsetDateTime::now_utc()),
                deleted_date: None,
            }),
            sleep_center: Some(140755855671306),
            test_date: Some(Date::from_calendar_date(2021, Month::May, 27).unwrap()),
            tests: vec![SleepOrderTest {
                id: 140756665106487,
                code: None,
                name: "test".to_string(),
                practice: Some(140756660256772),
                created_date: Some(OffsetDateTime::now_utc()),
                deleted_date: None,
            }],
        }
    }
}
