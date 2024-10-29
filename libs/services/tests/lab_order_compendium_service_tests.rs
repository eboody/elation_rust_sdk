#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::orders::*;
    use services::orders::LabOrderCompendiumService;
    use services::prelude::*;

    use serial_test::serial;
    use time::OffsetDateTime;

    #[serial]
    #[tokio::test]
    async fn test_get_lab_order_compendium_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /lab_order_compendiums/{id}/ endpoint
        let compendium_id = 140745672294843;
        let lab_order_compendium = get_mock_lab_order_compendium(compendium_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/lab_order_compendiums/{}/", compendium_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&lab_order_compendium).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderCompendiumService::new(&client);

        // Call the method under test
        let result = service.get(compendium_id).await;

        // Assert the result
        assert!(result.is_ok());
        let compendium = result.unwrap();
        assert_eq!(compendium.id, compendium_id);
        assert_eq!(compendium.name, "labcorp");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_lab_order_compendiums_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let mock_compendium = get_mock_lab_order_compendium(140745672294843);
        let compendiums = vec![mock_compendium.clone()];
        let compendiums_json = serde_json::to_string(&compendiums).unwrap();

        // Prepare query parameters
        let query_params = LabOrderCompendiumQueryParams {
            ..Default::default()
        };

        // Mock the GET /lab_order_compendiums/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/lab_order_compendiums/");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {compendiums_json}, \"next\": null, \"previous\": null, \"count\": 1 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderCompendiumService::new(&client);

        // Call the method under test
        let result = service.find(query_params).await;

        // Assert the result
        assert!(result.is_ok());

        let compendiums = result.unwrap().results;
        assert_eq!(compendiums.len(), 1);
        assert_eq!(compendiums[0].id, 140745672294843);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_lab_order_compendium_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the compendium data to create
        let compendium_for_create = LabOrderCompendiumForCreate {
            lab_vendor: 67186196726,
            code: "new_compendium".to_string(),
            name: "New Compendium".to_string(),
        };

        let created_compendium = LabOrderCompendium {
            id: 140745672294844,
            lab_vendor: 67186196726,
            code: "new_compendium".to_string(),
            name: "New Compendium".to_string(),
            last_updated: OffsetDateTime::now_utc(),
            created_date: OffsetDateTime::now_utc(),
            deleted_date: None,
        };

        // Mock the POST /lab_order_compendiums/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/lab_order_compendiums")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&compendium_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&created_compendium).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderCompendiumService::new(&client);

        // Call the method under test
        let result = service.post(&compendium_for_create).await;

        println!("{result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let compendium = result.unwrap();
        assert_eq!(compendium.id, 140745672294844);
        assert_eq!(compendium.name, "New Compendium");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    //#[serial]
    //#[tokio::test]
    //async fn test_update_lab_order_compendium_success_put() {
    //    // Start a local mock server
    //    let server = MockServer::start_async().await;
    //
    //    // Set the mock environment variables
    //    std::env::set_var("TEST_ENV", "TRUE");
    //    std::env::set_var("MOCK_SERVER_URL", server.base_url());
    //
    //    // Prepare the compendium data to update (PUT)
    //    let compendium_for_update = LabOrderCompendiumForUpdate {
    //        lab_vendor: Some(67186196726),
    //        code: Some("updated_compendium".to_string()),
    //        name: Some("Updated Compendium".to_string()),
    //    };
    //
    //    let updated_compendium = LabOrderCompendium {
    //        id: 140745672294843,
    //        lab_vendor: 67186196726,
    //        code: "updated_compendium".to_string(),
    //        name: "Updated Compendium".to_string(),
    //        last_updated: OffsetDateTime::now_utc(),
    //        created_date: OffsetDateTime::now_utc(),
    //        deleted_date: None,
    //    };
    //
    //    // Mock the PUT /lab_order_compendiums/{id}/ endpoint
    //    let mock = server.mock(|when, then| {
    //        when.method(PUT)
    //            .path(format!("/lab_order_compendiums/{}/", updated_compendium.id))
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&compendium_for_update).unwrap());
    //        then.status(200)
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&updated_compendium).unwrap());
    //    });
    //
    //    // Create a client pointing to the mock server
    //    let client = Client::new().await.unwrap();
    //    let service = LabOrderCompendiumService::new(&client);
    //
    //    // Call the method under test
    //    let result = service
    //        .put(updated_compendium.id, &compendium_for_update)
    //        .await;
    //
    //    // Assert the result
    //    assert!(result.is_ok());
    //    let compendium = result.unwrap();
    //    assert_eq!(compendium.id, updated_compendium.id);
    //    assert_eq!(compendium.name, "Updated Compendium");
    //
    //    // Ensure the mock was called
    //    mock.assert_async().await;
    //}

    #[serial]
    #[tokio::test]
    async fn test_update_lab_order_compendium_success_patch() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Prepare the compendium data to update (PATCH)
        let compendium_for_update = LabOrderCompendiumForUpdate {
            name: Some("Partially Updated Compendium".to_string()),
            ..Default::default()
        };

        let updated_compendium = LabOrderCompendium {
            id: 140745672294843,
            lab_vendor: 67186196726,
            code: "labcorp".to_string(),
            name: "Partially Updated Compendium".to_string(),
            last_updated: OffsetDateTime::now_utc(),
            created_date: OffsetDateTime::now_utc(),
            deleted_date: None,
        };

        // Mock the PATCH /lab_order_compendiums/{id}/ endpoint
        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/lab_order_compendiums/{}/", updated_compendium.id))
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&compendium_for_update).unwrap());
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&updated_compendium).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderCompendiumService::new(&client);

        // Call the method under test
        let result = service
            .patch(updated_compendium.id, &compendium_for_update)
            .await;

        // Assert the result
        assert!(result.is_ok());
        let compendium = result.unwrap();
        assert_eq!(compendium.id, updated_compendium.id);
        assert_eq!(compendium.name, "Partially Updated Compendium");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_lab_order_compendium_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock environment variables
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the DELETE /lab_order_compendiums/{id}/ endpoint
        let compendium_id = 140745672294843;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/lab_order_compendiums/{}/", compendium_id));
            then.status(204); // No Content
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = LabOrderCompendiumService::new(&client);

        // Call the method under test
        let result = service.delete(compendium_id).await;

        // Assert the result
        assert!(result.is_ok());

        // Ensure the mock was called
        mock.assert_async().await;
    }

    // Helper function to create a mock LabOrderCompendium
    fn get_mock_lab_order_compendium(compendium_id: i64) -> LabOrderCompendium {
        LabOrderCompendium {
            id: compendium_id,
            lab_vendor: 67186196726,
            name: "labcorp".to_string(),
            code: "labcorp".to_string(),
            last_updated: OffsetDateTime::now_utc(),
            created_date: OffsetDateTime::now_utc(),
            deleted_date: None,
        }
    }
}
