#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::LabOrderService;
    use services::prelude::*;
    use time::{Date, OffsetDateTime};

    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_lab_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /lab_orders/{id}/ endpoint
        let order_id = 140754512183329;
        let lab_order = get_mock_lab_order(order_id);

        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("/lab_orders/{}/", order_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&lab_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderService::new(&client);

        // Call the method under test
        let result = service.get(order_id).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, order_id);
        assert_eq!(order.patient, 140754511659009);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_lab_order_success() {
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the lab order data to create
        let order_for_create = LabOrderForCreate {
            patient: 140754511659009,
            practice: 140754506678276,
            ordering_physician: 140754510217218,
            chart_date: Some(OffsetDateTime::now_utc()),
            document_date: Some(OffsetDateTime::now_utc()),
            confidential: Some(false),
            follow_up_method: Some("email".to_string()),
            resolution: Some(ResolutionForCreate {
                state: ResolutionState::Outstanding,
                resolving_document: Some(140754512183329),
            }),
            test_date: Some(Date::from_calendar_date(2021, time::Month::March, 25).unwrap()),
            vendor: Some(67191701750),
            content: Some(LabOrderContentForCreate {
                tests: vec![LabOrderTestForCreate {
                    id: 140748306251838,
                }],
                fasting_method: Some(FastingMethod::FastingRandom),
                patient_instructions: Some("Please fast for 8 hours before the test.".to_string()),
                test_center_notes: Some("Handle with care.".to_string()),
                stat_method: Some(StatMethod::StatFax),
                icd10_codes: Some(vec![Icd10Code {
                    code: "E11.9".to_string(),
                    description: Some("Type 2 diabetes mellitus without complications".to_string()),
                }]),
                standing_order_frequency: None,
                standing_order_end_date: None,
                collection_datetime: None,
            }),
            ccs: Some(vec![131074]),
            bill_type: Some(BillType::Patient),
            answers: Some(vec![AnswerForCreate {
                test: 140748306251838,
                question: 123456,
                value: "Answer to question".to_string(),
            }]),
            site: Some(12345),
            tags: Some(vec![67890]),
        };

        let created_order = get_mock_lab_order(140754512183330);

        // Mock the POST /lab_orders/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/lab_orders")
                .header("Content-Type", "application/json")
                .json_body_obj(&order_for_create);
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&created_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderService::new(&client);

        // Call the method under test
        let result = service.post(&order_for_create).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, 140754512183330);
        assert_eq!(order.patient, 140754511659009);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_lab_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let order_id = 140754512183329;

        // Prepare the lab order data to update
        let order_for_update = LabOrderForUpdate {
            follow_up_method: Some("phone".to_string()),
            confidential: Some(true),
            tags: Some(vec![67890, 67891]),
            ..Default::default()
        };

        let updated_order = LabOrder {
            follow_up_method: Some("phone".to_string()),
            confidential: true,
            tags: vec![],
            ..get_mock_lab_order(order_id)
        };

        // Mock the PATCH /lab_orders/{id}/ endpoint
        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/lab_orders/{}/", order_id))
                .header("Content-Type", "application/json")
                .json_body_obj(&order_for_update);
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&updated_order).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderService::new(&client);

        // Call the method under test
        let result = service.patch(order_id, &order_for_update).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, order_id);
        // Assuming the mock data reflects the update
        assert!(order.confidential);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_lab_order_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the DELETE /lab_orders/{id}/ endpoint
        let order_id = 140754512183329;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/lab_orders/{}/", order_id));
            then.status(204); // No Content
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderService::new(&client);

        // Call the method under test
        let result = service.delete(order_id).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_lab_orders_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let order1 = get_mock_lab_order(140754512183329);
        let mut order2 = get_mock_lab_order(140754512183330);
        order2.patient = 140754511659010; // Different patient

        let vec_of_orders = serde_json::to_string(&vec![order1.clone(), order2.clone()]).unwrap();

        // Mock the GET /lab_orders/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/lab_orders/")
                .query_param("patient", "140754511659009")
                .query_param("unsigned", "false");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {vec_of_orders}, \"next\": null, \"previous\": null, \"count\": 2 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderService::new(&client);

        // Prepare query parameters
        let query_params = LabOrderQueryParams {
            patient: Some(140754511659009),
            unsigned: Some(false),
            ..Default::default()
        };

        // Call the method under test
        let result = service.find(query_params).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let orders = result.unwrap().results;
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].patient, 140754511659009);
        // The second order has a different patient ID; adjust assertions accordingly

        // Ensure the mock was called
        mock.assert_async().await;
    }

    // Helper function to create a mock lab order
    fn get_mock_lab_order(order_id: i64) -> LabOrder {
        LabOrder {
            id: order_id,
            ccs: vec![],
            chart_date: Some(
                OffsetDateTime::parse(
                    "2021-03-25T04:33:40Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            confidential: false,
            content: LabOrderContent {
                id: 140754512117861,
                fasting_method: Some(FastingMethod::FastingRandom),
                patient_instructions: Some("".to_string()),
                test_center_notes: Some("".to_string()),
                stat_method: Some(StatMethod::StatFax),
                standing_order_frequency: None,
                standing_order_end_date: None,
                collection_datetime: None,
                icd10_codes: vec![],
                tests: vec![LabOrderTest {
                    id: 140748306251838,
                    name: "TSH".to_string(),
                    code: None,
                    procedure_class: None,
                    practice_created: None,
                    lab_vendor: None,
                    compendium: None,
                    cpts: vec![],
                    synonyms: vec![],
                    questions: vec![],
                    created_date: Some(
                        OffsetDateTime::parse(
                            "2020-01-27T16:17:43Z",
                            &time::format_description::well_known::Rfc3339,
                        )
                        .unwrap(),
                    ),
                    deleted_date: None,
                }],
            },
            created_date: Some(
                OffsetDateTime::parse(
                    "2021-03-25T04:33:40Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            document_date: Some(
                OffsetDateTime::parse(
                    "2021-03-25T04:33:40Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            follow_up_method: Some("".to_string()),
            ordering_physician: 140754510217218,
            patient: 140754511659009,
            practice: 140754506678276,
            requisition: Some(140754512183329),
            resolution: Some(Resolution {
                id: 140754511659009,
                document: order_id,
                resolving_document: Some(order_id),
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
            signed_by: Some(140754510217218),
            signed_date: Some(
                OffsetDateTime::parse(
                    "2021-03-25T04:33:40Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            site: None,
            special_insurance: None,
            submissions: vec![],
            specimens: vec![],
            tags: vec![],
            test_date: None,
            vendor: 67191701750,
            printable_view: Some(
                "http://127.0.0.1/api/2.0/lab_orders/140758848962593/printable".to_string(),
            ),
            bill_type: None,
            answers: vec![],
            facility: None,
        }
    }
}
