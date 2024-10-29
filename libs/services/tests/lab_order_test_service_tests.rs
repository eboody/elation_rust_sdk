#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::LabOrderTestService;
    use services::prelude::*;

    use serial_test::serial;
    use time::OffsetDateTime;

    #[serial]
    #[tokio::test]
    async fn test_get_lab_order_test_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /lab_order_tests/{id}/ endpoint
        let test_id = 140756679458878;
        let lab_order_test = get_mock_lab_order_test(test_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/lab_order_tests/{}/", test_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&lab_order_test).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderTestService::new(&client);

        // Call the method under test
        let result = service.get(test_id).await;

        // Assert the result
        assert!(result.is_ok());
        let test = result.unwrap();
        assert_eq!(test.id, test_id);
        assert_eq!(test.name, "Microalbumin:creatinine ratio, random urine");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    // Additional test cases for find, create, and delete can be added similarly.

    fn get_mock_lab_order_test(test_id: i64) -> LabOrderTest {
        LabOrderTest {
            id: test_id,
            code: "DLS546".to_string(),
            compendium: 140745672360379,
            cpts: vec!["90387".to_string()],
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
            lab_vendor: 63929778422,
            name: "Microalbumin:creatinine ratio, random urine".to_string(),
            practice_created: Some(140756679131140),
            procedure_class: Some("MI".to_string()),
            questions: vec![Question {
                test: test_id,
                sequence: 1,
                required: true,
                question: QuestionDetails {
                    id: 67223159186,
                    code: "Q002".to_string(),
                    value: "CSF Tube #".to_string(),
                    question_type: "enum".to_string(),
                    secondary_code: None,
                    example_format: None,
                    max_length: None,
                    choices: vec![
                        Choice {
                            id: 67222897041,
                            code: "A0602".to_string(),
                            value: "CSF, Tube 2".to_string(),
                            created_date: Some(OffsetDateTime::now_utc()),
                            deleted_date: None,
                        },
                        // Additional choices...
                    ],
                    created_date: Some(OffsetDateTime::now_utc()),
                    deleted_date: None,
                },
            }],
            synonyms: vec![
                "Biopsy".to_string(),
                "Gross and Microscopic Pathology".to_string(),
                // Additional synonyms...
            ],
        }
    }
}
