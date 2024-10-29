#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::LabVendorService;
    use services::prelude::*;

    use serial_test::serial;
    use time::OffsetDateTime;

    #[serial]
    #[tokio::test]
    async fn test_get_lab_vendor_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /lab_vendors/{id}/ endpoint
        let vendor_id = 63929778422;
        let lab_vendor = get_mock_lab_vendor(vendor_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/lab_vendors/{}/", vendor_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&lab_vendor).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabVendorService::new(&client);

        // Call the method under test
        let result = service.get(vendor_id).await;

        // Assert the result
        assert!(result.is_ok());
        let vendor = result.unwrap();
        assert_eq!(vendor.id, vendor_id);
        assert_eq!(vendor.name, "DLS");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_lab_vendors_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let mock_vendor = get_mock_lab_vendor(63929778422);
        let vendors = vec![mock_vendor.clone()];
        let vendors_json = serde_json::to_string(&vendors).unwrap();

        // Mock the GET /lab_vendors/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/lab_vendors/")
                .query_param("name", "DLS");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {vendors_json}, \"next\": null, \"previous\": null, \"count\": 1 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabVendorService::new(&client);

        // Prepare query parameters
        let query_params = LabVendorQueryParams {
            name: Some(vec!["DLS".to_string()]),
            ..Default::default()
        };

        // Call the method under test
        let result = service.find(query_params).await;

        // Assert the result
        assert!(result.is_ok());
        let vendors = result.unwrap().results;
        assert_eq!(vendors.len(), 1);
        assert_eq!(vendors[0].id, 63929778422);
        assert_eq!(vendors[0].name, "DLS");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_lab_vendor_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the vendor data to create
        let vendor_for_create = LabVendorForCreate {
            name: "Lab".to_string(),
            display_name: "New Lab Vendor".to_string(),
            practice_created: 123456789,
            has_test_compendium: true,
            results_integration_available: true,
            orders_integration_available: true,
        };

        let created_vendor = LabVendor {
            id: 987654321,
            practice_created: Some(123456789),
            name: "Lab".to_string(),
            display_name: "New Lab Vendor".to_string(),
            has_order_compendium: true,
            has_test_compendium: true,
            results_integration_available: true,
            orders_integration_available: true,
            compendiums: vec![],
            default_compendium: None,
        };

        // Mock the POST /lab_vendors/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/lab_vendors/")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&vendor_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&created_vendor).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabVendorService::new(&client);

        // Call the method under test
        let result = service.post(&vendor_for_create).await;

        // Assert the result
        assert!(result.is_ok());
        let vendor = result.unwrap();
        assert_eq!(vendor.id, created_vendor.id);
        assert_eq!(vendor.name, "Lab");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    //#[serial]
    //#[tokio::test]
    //async fn test_update_lab_vendor_success_put() {
    //    // Start a local mock server
    //    let server = MockServer::start_async().await;
    //
    //    // Set the mock environment variables
    //    std::env::set_var("TEST_ENV", "TRUE");
    //    std::env::set_var("MOCK_SERVER_URL", server.base_url());
    //
    //    // Prepare the vendor data to update (PUT)
    //    let vendor_for_update = LabVendorForUpdate {
    //        name: Some("Lab".to_string()),
    //        display_name: Some("Updated Lab Vendor".to_string()),
    //        practice_created: Some(123456789),
    //        has_test_compendium: Some(true),
    //        results_integration_available: Some(true),
    //        orders_integration_available: Some(true),
    //    };
    //
    //    let updated_vendor = LabVendor {
    //        id: 63929778422,
    //        practice_created: Some(123456789),
    //        name: "Lab".to_string(),
    //        display_name: "Updated Lab Vendor".to_string(),
    //        has_order_compendium: true,
    //        has_test_compendium: true,
    //        results_integration_available: true,
    //        orders_integration_available: true,
    //        compendiums: vec![],
    //        default_compendium: None,
    //    };
    //
    //    // Mock the PUT /lab_vendors/{id}/ endpoint
    //    let mock = server.mock(|when, then| {
    //        when.method(PUT)
    //            .path(format!("/lab_vendors/{}/", updated_vendor.id))
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&vendor_for_update).unwrap());
    //        then.status(200)
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&updated_vendor).unwrap());
    //    });
    //
    //    // Create a client pointing to the mock server
    //    let client = Client::new().await.unwrap();
    //    let service = LabVendorService::new(&client);
    //
    //    // Call the method under test
    //    let result = service.put(updated_vendor.id, &vendor_for_update).await;
    //
    //    // Assert the result
    //    assert!(result.is_ok());
    //    let vendor = result.unwrap();
    //    assert_eq!(vendor.id, updated_vendor.id);
    //    assert_eq!(vendor.display_name, "Updated Lab Vendor");
    //
    //    // Ensure the mock was called
    //    mock.assert_async().await;
    //}

    #[serial]
    #[tokio::test]
    async fn test_update_lab_vendor_success_patch() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the vendor data to update (PATCH)
        let vendor_for_update = LabVendorForUpdate {
            display_name: Some("Partially Updated Lab Vendor".to_string()),
            ..Default::default()
        };

        let updated_vendor = LabVendor {
            id: 63929778422,
            practice_created: None,
            name: "DLS".to_string(),
            display_name: "Partially Updated Lab Vendor".to_string(),
            has_order_compendium: true,
            has_test_compendium: true,
            results_integration_available: true,
            orders_integration_available: true,
            compendiums: vec![],
            default_compendium: None,
        };

        // Mock the PATCH /lab_vendors/{id}/ endpoint
        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/lab_vendors/{}/", updated_vendor.id))
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&vendor_for_update).unwrap());
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&updated_vendor).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabVendorService::new(&client);

        // Call the method under test
        let result = service.patch(updated_vendor.id, &vendor_for_update).await;

        // Assert the result
        assert!(result.is_ok());
        let vendor = result.unwrap();
        assert_eq!(vendor.id, updated_vendor.id);
        assert_eq!(vendor.display_name, "Partially Updated Lab Vendor");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_lab_vendor_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the DELETE /lab_vendors/{id}/ endpoint
        let vendor_id = 63929778422;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/lab_vendors/{}/", vendor_id));
            then.status(204); // No Content
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabVendorService::new(&client);

        // Call the method under test
        let result = service.delete(vendor_id).await;

        // Assert the result
        assert!(result.is_ok());

        // Ensure the mock was called
        mock.assert_async().await;
    }

    // Helper function to create a mock LabVendor
    fn get_mock_lab_vendor(vendor_id: i64) -> LabVendor {
        LabVendor {
            id: vendor_id,
            practice_created: None,
            name: "DLS".to_string(),
            display_name: "DLS".to_string(),
            has_order_compendium: true,
            has_test_compendium: true,
            results_integration_available: true,
            orders_integration_available: true,
            compendiums: vec![LabOrderCompendium {
                id: 140758847586747,
                lab_vendor: 63929778422,
                name: "Ariana Jones".to_string(),
                code: "Sean Howard".to_string(),
                last_updated: OffsetDateTime::now_utc(),
                created_date: OffsetDateTime::now_utc(),
                deleted_date: None,
            }],
            default_compendium: Some(LabOrderCompendium {
                id: 140745672360379,
                lab_vendor: 67186196726,
                name: "Harris10 PLC2022-05-06 10:54:18+00:00 Compendium".to_string(),
                code: "Harris1 PLC2022-05-06 10:54:18+00:00".to_string(),
                last_updated: OffsetDateTime::now_utc(),
                created_date: OffsetDateTime::now_utc(),
                deleted_date: Some(OffsetDateTime::now_utc()),
            }),
        }
    }
}
