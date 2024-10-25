#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::orders::{LabOrder, LabOrderForCreate, LabOrderForUpdate};
    use orders::LabOrderService;
    use serial_test::serial;
    use services::*;
    use time::OffsetDateTime;

    fn get_mock_lab_order(lab_order_id: i64) -> LabOrder {
        LabOrder {
            id: lab_order_id,
            bill_type: Some("thirdparty".to_string()),
            chart_date: OffsetDateTime::now_utc(),
            confidential: false,
            content: None, // Populate as needed
            created_date: OffsetDateTime::now_utc(),
            document_date: OffsetDateTime::now_utc(),
            ordering_physician: 12345,
            patient: 67890,
            practice: 54321,
            requisition: lab_order_id,
            resolution: None,
            signed_by: 12345,
            signed_date: OffsetDateTime::now_utc(),
            vendor: 98765,
            printable_view: Some("http://example.com".to_string()),
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_lab_order_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let lab_order_id = 123456;
        let lab_order = get_mock_lab_order(lab_order_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/lab_orders/{}/", lab_order_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&lab_order).unwrap());
        });

        let client = Client::new().await.unwrap();
        let lab_order_service = LabOrderService::new(&client);

        let result = lab_order_service.get(lab_order_id).await;

        assert!(result.is_ok());
        let fetched_lab_order = result.unwrap();
        assert_eq!(fetched_lab_order.id, lab_order_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_lab_order_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let lab_order_id = 789012;
        let lab_order_for_create = LabOrderForCreate {
            patient: 67890,
            ordering_physician: 12345,
            practice: 54321,
            vendor: 98765,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/lab_orders")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&lab_order_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_lab_order(lab_order_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = LabOrderService::new(&client);

        let result = service.create(&lab_order_for_create).await;

        assert!(result.is_ok());
        let created_lab_order = result.unwrap();
        assert_eq!(created_lab_order.id, lab_order_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_lab_order_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let lab_order_id = 123456;
        let mock_lab_order = LabOrder {
            signed_by: 67890,
            ..get_mock_lab_order(lab_order_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/lab_orders/{}/", lab_order_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "signed_by": 67890
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_lab_order).unwrap());
        });

        let client = Client::new().await.unwrap();
        let lab_order_service = LabOrderService::new(&client);

        let lab_order_fu = LabOrderForUpdate {
            signed_by: Some(67890),
            signed_date: Some(OffsetDateTime::now_utc()),
            ..LabOrderForUpdate::default()
        };

        let result = lab_order_service.update(lab_order_id, &lab_order_fu).await;

        assert!(result.is_ok());
        let updated_lab_order = result.unwrap();
        assert_eq!(updated_lab_order.id, lab_order_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_lab_order_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let lab_order_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/lab_orders/{}/", lab_order_id));
            then.status(204);
        });

        let client = Client::new().await.unwrap();
        let lab_order_service = LabOrderService::new(&client);

        let result = lab_order_service.delete(lab_order_id).await;

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
