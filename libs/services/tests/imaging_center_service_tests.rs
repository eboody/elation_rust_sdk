#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::ImagingCenterService;
    use services::prelude::*;
    use time::OffsetDateTime;

    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_imaging_center_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /imaging_centers/{id}/ endpoint
        let center_id = 140755855671306;
        let imaging_center = get_mock_imaging_center(center_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/imaging_centers/{}/", center_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&imaging_center).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = ImagingCenterService::new(&client);

        // Call the method under test
        let result = service.get(center_id).await;

        println!("result: {:#?}", result);

        // Assert the result
        assert!(result.is_ok());
        let center = result.unwrap();
        assert_eq!(center.id, center_id);
        assert_eq!(center.location_name, "Test Imaging Center");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_imaging_centers_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let center1 = get_mock_imaging_center(140755855671306);
        let mut center2 = get_mock_imaging_center(140755855671307);
        center2.location_name = "Another Imaging Center".to_string();

        let vec_of_centers =
            serde_json::to_string(&vec![center1.clone(), center2.clone()]).unwrap();

        // Mock the GET /imaging_centers/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/imaging_centers/")
                .query_param("practice", "65540");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {vec_of_centers}, \"next\": null, \"previous\": null, \"count\": 2 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = ImagingCenterService::new(&client);

        // Prepare query parameters
        let query_params = ImagingCenterQueryParams {
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

    // Helper function to create a mock imaging center
    fn get_mock_imaging_center(center_id: i64) -> ImagingCenter {
        ImagingCenter {
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
            location_name: "Test Imaging Center".to_string(),
            phone: Some("555-555-5555".to_string()),
            practice: Some(65540),
            state: "CA".to_string(),
            zip: "94103".to_string(),
        }
    }
}
