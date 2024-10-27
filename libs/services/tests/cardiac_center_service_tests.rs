#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::CardiacCenterService;
    use services::prelude::*;
    use time::OffsetDateTime;

    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_cardiac_center_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /cardiac_centers/{id}/ endpoint
        let center_id = 140755855671306;
        let cardiac_center = get_mock_cardiac_center(center_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/cardiac_centers/{}/", center_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&cardiac_center).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = CardiacCenterService::new(&client);

        // Call the method under test
        let result = service.get(center_id).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let center = result.unwrap();
        assert_eq!(center.id, center_id);
        assert_eq!(center.location_name, "Test Cardiac Center");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    //#[serial]
    //#[tokio::test]
    //async fn test_create_cardiac_center_success() {
    //    let server = MockServer::start_async().await;
    //
    //    // Set the mock environment variables
    //    std::env::set_var("TEST_ENV", "TRUE");
    //    std::env::set_var("MOCK_SERVER_URL", server.base_url());
    //
    //    // Prepare the cardiac center data to create
    //    let center_for_create = CardiacCenterForCreate {
    //        address_line1: "123 Elation St".to_string(),
    //        address_line2: Some("Suite 4".to_string()),
    //        city: "San Francisco".to_string(),
    //        company_name: "Test Ancillary Company".to_string(),
    //        company: 140755855605768,
    //        fax: Some("444-444-4444".to_string()),
    //        location_name: "Test Cardiac Center".to_string(),
    //        phone: Some("555-555-5555".to_string()),
    //        practice: 65540,
    //        state: "CA".to_string(),
    //        zip: "94103".to_string(),
    //    };
    //
    //    let created_center = get_mock_cardiac_center(140755855671307);
    //
    //    // Mock the POST /cardiac_centers/ endpoint
    //    let mock = server.mock(|when, then| {
    //        when.method(POST)
    //            .path("/cardiac_centers")
    //            .header("Content-Type", "application/json")
    //            .json_body_obj(&center_for_create);
    //        then.status(201)
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&created_center).unwrap());
    //    });
    //
    //    // Create a client pointing to the mock server
    //    let client = Client::new().await.unwrap();
    //    let service = CardiacCenterService::new(&client);
    //
    //    // Call the method under test
    //    let result = service.post(&center_for_create).await;
    //
    //    println!("result: {:#?}", result);
    //
    //    // Assert the result
    //    assert!(result.is_ok());
    //    let center = result.unwrap();
    //    assert_eq!(center.id, 140755855671307);
    //    assert_eq!(center.location_name, "Test Cardiac Center");
    //
    //    // Ensure the mock was called
    //    mock.assert_async().await;
    //}

    #[serial]
    #[tokio::test]
    async fn test_find_cardiac_centers_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let center1 = get_mock_cardiac_center(140755855671306);
        let mut center2 = get_mock_cardiac_center(140755855671307);
        center2.location_name = "Another Cardiac Center".to_string();

        let vec_of_centers =
            serde_json::to_string(&vec![center1.clone(), center2.clone()]).unwrap();

        // Mock the GET /cardiac_centers/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/cardiac_centers/")
                .query_param("practice", "65540");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {vec_of_centers}, \"next\": null, \"previous\": null, \"count\": 2 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = CardiacCenterService::new(&client);

        // Prepare query parameters
        let query_params = CardiacCenterQueryParams {
            practice: Some(65540),
            ..Default::default()
        };

        // Call the method under test
        let result = service.find(query_params).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let centers = result.unwrap().results;
        assert_eq!(centers.len(), 2);
        assert_eq!(centers[0].practice, Some(65540));
        assert_eq!(centers[1].practice, Some(65540));

        // Ensure the mock was called
        mock.assert_async().await;
    }

    // Helper function to create a mock cardiac center
    fn get_mock_cardiac_center(center_id: i64) -> CardiacCenter {
        CardiacCenter {
            id: center_id,
            address_line1: "123 Elation St".to_string(),
            address_line2: Some("Suite 4".to_string()),
            city: "San Francisco".to_string(),
            company_name: "Test Ancillary Company".to_string(),
            company: 140755855605768,
            created_date: Some(
                OffsetDateTime::parse(
                    "2016-10-10T23:31:49Z",
                    &time::format_description::well_known::Rfc3339,
                )
                .unwrap(),
            ),
            deleted_date: None,
            fax: Some("444-444-4444".to_string()),
            location_name: "Test Cardiac Center".to_string(),
            phone: Some("555-555-5555".to_string()),
            practice: Some(65540),
            state: "CA".to_string(),
            zip: "94103".to_string(),
        }
    }
}
