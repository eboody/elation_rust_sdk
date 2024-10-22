#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::{Vaccine, VaccineForCreate, VaccineForUpdate};
    use serial_test::serial;
    use services::{patient_profile::VaccineService, ResourceService};
    use time::OffsetDateTime;

    fn get_mock_vaccine(vaccine_id: i64) -> Vaccine {
        Vaccine {
            id: vaccine_id,
            description: Some("Td(adult) unspecified formulation (Td)".to_string()),
            name: Some("Td(adult) unspecified formulation".to_string()),
            cvx: 139,
            cdc_type: Some("Td".to_string()),
            ndc: Some(true),
            cdc_name: None,
            ndc_values: Some(64058687489),
            practice: Some("".to_string()),
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_vaccine_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let vaccine_id = 123456;
        let vaccine = get_mock_vaccine(vaccine_id);

        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("/vaccines/{}/", vaccine_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&vaccine).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let vaccine_service = VaccineService::new(&client);

        // Call the method under test
        let result = vaccine_service.get(vaccine_id).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let fetched_vaccine = result.unwrap();
        assert_eq!(fetched_vaccine.id, vaccine_id);
        assert_eq!(
            fetched_vaccine.name,
            Some("Td(adult) unspecified formulation".to_owned())
        );

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_vaccine_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let vaccine_id = 789012;
        let vaccine_for_create = VaccineForCreate {
            description: "Td(adult) unspecified formulation (Td)".to_string(),
            name: "Td(adult) unspecified formulation".to_string(),
            cdc_name: None,
            cvx: 139,
            cdc_type: "Td".to_string(),
            ndc: true,
            ndc_values: Some(64058687489),
            practice: Some("".to_string()),
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/vaccines")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&vaccine_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_vaccine(vaccine_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = VaccineService::new(&client);

        let result = service.create(&vaccine_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_vaccine = result.unwrap();
        assert_eq!(created_vaccine.id, vaccine_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_vaccine_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let vaccine_id = 123456;
        let mock_vaccine = Vaccine {
            name: Some("Updated Td(adult)".to_owned()),
            ..get_mock_vaccine(vaccine_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/vaccines/{}/", vaccine_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "name": "Updated Td(adult)"
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_vaccine).unwrap());
        });

        let client = Client::new().await.unwrap();
        let vaccine_service = VaccineService::new(&client);

        let vaccine_fu = VaccineForUpdate {
            name: Some("Updated Td(adult)".to_owned()),
            ..VaccineForUpdate::default()
        };

        let result = vaccine_service.update(vaccine_id, &vaccine_fu).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let updated_vaccine = result.unwrap();
        assert_eq!(updated_vaccine.id, vaccine_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_vaccine_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let vaccine_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/vaccines/{}/", vaccine_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let vaccine_service = VaccineService::new(&client);

        let result = vaccine_service.delete(vaccine_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
