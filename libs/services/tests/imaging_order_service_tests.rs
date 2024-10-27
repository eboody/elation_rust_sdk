#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, POST, PUT};
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::ImagingOrderService;
    use services::prelude::*;
    use time::{Date, OffsetDateTime};

    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_imaging_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /imaging_orders/{id}/ endpoint
        let order_id = 140756377075740;
        let imaging_order = get_mock_imaging_order(order_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/imaging_orders/{}/", order_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&imaging_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = ImagingOrderService::new(&client);

        // Call the method under test
        let result = service.get(order_id).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, order_id);
        assert_eq!(order.patient, 140756664516609);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_imaging_order_success() {
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the imaging order data to create
        let order_for_create = ImagingOrderForCreate {
            ancillary_company: 140755855605768,
            ccs: Some(vec![131074]),
            chart_date: Some(OffsetDateTime::now_utc()),
            clinical_reason: "sickness".to_string(),
            confidential: Some(true),
            document_date: Some(OffsetDateTime::now_utc()),
            follow_up_method: Some("email".to_string()),
            icd10_codes: Some(vec![Icd10Code {
                code: "O09.529".to_string(),
                description: "Supervision of elderly multigravida, unspecified trimester"
                    .to_string(),
            }]),
            imaging_center: Some(140755855671306),
            patient: 140756664516609,
            practice: 140756660256772,
            prescribing_user: 2032,
            statmethod: Some(StatMethod::WetReadingFax),
            tests: Some(vec![]),
            test_date: Some(Date::from_calendar_date(2021, time::Month::May, 27).unwrap()),
        };

        let created_order = get_mock_imaging_order(140756377075741);

        // Mock the POST /imaging_orders/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/imaging_orders")
                .header("Content-Type", "application/json")
                .json_body_obj(&order_for_create);
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&created_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = ImagingOrderService::new(&client);

        // Call the method under test
        let result = service.post(&order_for_create).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, 140756377075741);
        assert_eq!(order.patient, 140756664516609);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_put_imaging_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let order_id = 140756377075740;

        // Prepare the imaging order data to update using ImagingOrderForCreate
        let order_for_update = ImagingOrderForCreate {
            ancillary_company: 140755855605768,
            ccs: Some(vec![131074]),
            chart_date: Some(OffsetDateTime::now_utc()),
            clinical_reason: "updated reason".to_string(),
            confidential: Some(false),
            document_date: Some(OffsetDateTime::now_utc()),
            follow_up_method: Some("phone".to_string()),
            icd10_codes: Some(vec![Icd10Code {
                code: "A00".to_string(),
                description: "Cholera".to_string(),
            }]),
            imaging_center: Some(140755855671306),
            patient: 140756664516609,  // Cannot be changed
            practice: 140756660256772, // Cannot be changed
            prescribing_user: 2032,
            statmethod: Some(StatMethod::WetReadingPhone),
            tests: Some(vec![]),
            test_date: Some(Date::from_calendar_date(2021, time::Month::June, 1).unwrap()),
        };

        let updated_order = get_mock_imaging_order(order_id);

        // Mock the PUT /imaging_orders/{id}/ endpoint
        let mock = server.mock(|when, then| {
            when.method(PUT)
                .path("/imaging_orders/")
                .header("Content-Type", "application/json")
                .json_body_obj(&order_for_update);
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&updated_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = ImagingOrderService::new(&client);

        // Call the method under test
        let result = service.put(&order_for_update).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, order_id);
        // The mock data has "sickness" as the clinical reason
        assert_eq!(order.clinical_reason, "sickness"); // Based on the mock data

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_imaging_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the DELETE /imaging_orders/{id}/ endpoint
        let order_id = 140756377075740;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/imaging_orders/{}/", order_id));
            then.status(204); // No Content
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = ImagingOrderService::new(&client);

        // Call the method under test
        let result = service.delete(order_id).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());

        // Ensure the mock was called
        mock.assert_async().await;
    }

    // Helper function to create a mock imaging order
    fn get_mock_imaging_order(order_id: i64) -> ImagingOrder {
        ImagingOrder {
            id: order_id,
            ancillary_company: 140755855605768,
            ccs: vec![131074],
            chart_date: Some(
                OffsetDateTime::parse(
                    "2021-05-26T03:49:04Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            clinical_reason: "sickness".to_string(),
            confidential: true,
            created_date: Some(
                OffsetDateTime::parse(
                    "2021-05-26T03:49:04Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            deleted_date: None,
            document_date: Some(
                OffsetDateTime::parse(
                    "2021-05-26T03:49:04Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            follow_up_method: Some("email".to_string()),
            icd10_codes: vec![Icd10Code {
                code: "O09.529".to_string(),
                description: "Supervision of elderly multigravida, unspecified trimester"
                    .to_string(),
            }],
            imaging_center: Some(140755855671306),
            patient: 140756664516609,
            practice: 140756660256772,
            prescribing_user: 2032,
            signed_date: Some(
                OffsetDateTime::parse(
                    "2021-05-26T03:49:04Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            signed_by: Some(1407563456436),
            resolution: Some(Resolution {
                id: 140754511659009,
                document: 140754512183329,
                resolving_document: Some(140754512183329),
                state: ResolutionState::Outstanding,
                note: None,
                created_date: Some(
                    OffsetDateTime::parse(
                        "2021-03-25T04:33:40Z",
                        &time::format_description::well_known::Rfc3339,
                    )
                    .unwrap(),
                ),
                deleted_date: None,
            }),
            statmethod: Some(StatMethod::WetReadingFax),
            test_date: Some(Date::from_calendar_date(2021, time::Month::May, 27).unwrap()),
            tests: vec![ImagingOrderTest {
                code: None,
                created_date: Some(
                    OffsetDateTime::parse(
                        "2021-05-26T03:49:04Z",
                        &time::format_description::well_known::Rfc3339,
                    )
                    .unwrap(),
                ),
                deleted_date: None,
                id: 140756665106487,
                name: "test".to_string(),
                practice: Some(140756660256772),
            }],
        }
    }
}