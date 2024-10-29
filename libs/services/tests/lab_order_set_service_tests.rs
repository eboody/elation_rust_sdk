#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::LabOrderSetService;
    use services::prelude::*;

    use serial_test::serial;
    use time::OffsetDateTime;

    #[serial]
    #[tokio::test]
    async fn test_get_lab_order_set_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /lab_order_sets/{id}/ endpoint
        let set_id = 24507383904;
        let lab_order_set = get_mock_lab_order_set(set_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/lab_order_sets/{}/", set_id));

            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&lab_order_set).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderSetService::new(&client);

        // Call the method under test
        let result = service.get(set_id).await;

        println!("{result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let set = result.unwrap();
        assert_eq!(set.id, set_id);
        assert_eq!(set.name, "Diabetes - Annual  (Order Set)");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    // Additional test cases for find, create, update, and delete can be added similarly.

    fn get_mock_lab_order_set(set_id: i64) -> LabOrderSet {
        LabOrderSet {
            id: set_id,
            compendium_code: Some("DLO".to_string()),
            content: LabOrderSetContent {
                id: 140754512117861,
                stat_method: None,
                patient_instructions: Some("".to_string()),
                test_center_notes: Some("".to_string()),
                fasting_method: FastingMethod::FastingRandom,
                standing_order_frequency: Some("".to_string()),
                standing_order_end_date: None,
                collection_datetime: None,
                icd10_codes: vec![],
                tests: vec![LabOrderTest {
                    id: 140748306251838,
                    name: "TSH".to_string(),
                    code: "".to_string(),
                    procedure_class: None,
                    practice_created: None,
                    lab_vendor: 1,
                    compendium: 1,
                    cpts: vec![],
                    synonyms: vec![],
                    questions: vec![],
                    created_date: Some(OffsetDateTime::now_utc()),
                    deleted_date: None,
                }],
            },
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
            lab_vendor: 67191701750,
            name: "Diabetes - Annual  (Order Set)".to_string(),
            practice: 1407566602,
        }
    }
}
