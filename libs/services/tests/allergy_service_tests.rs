#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::{Allergy, AllergyForCreate, AllergyForUpdate, AllergyStatus};
    use serial_test::serial;
    use services::*;
    use time::{Date, OffsetDateTime};

    fn get_mock_allergy(allergy_id: i64) -> Allergy {
        Allergy {
            id: allergy_id,
            status: AllergyStatus::Active,
            start_date: Date::from_calendar_date(1980, time::Month::January, 1).unwrap(),
            reaction: Some("nausea and vomiting".to_string()),
            name: "Erythromycin".to_string(),
            severity: None,
            medispanid: None,
            medispandnid: None,
            patient: 64072843265,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_allergy_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let allergy_id = 123456;
        let allergy = get_mock_allergy(allergy_id);

        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("/allergies/{}/", allergy_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&allergy).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let allergy_service = AllergyService::new(&client);

        // Call the method under test
        let result = allergy_service.get(allergy_id).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let fetched_allergy = result.unwrap();
        assert_eq!(fetched_allergy.id, allergy_id);
        assert_eq!(fetched_allergy.name, "Erythromycin");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_allergy_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let allergy_id = 789012;
        let allergy_for_create = AllergyForCreate {
            status: AllergyStatus::Active,
            start_date: Date::from_calendar_date(1980, time::Month::January, 1).unwrap(),
            reaction: Some("nausea and vomiting".to_string()),
            name: "Erythromycin".to_string(),
            severity: None,
            medispanid: None,
            medispandnid: None,
            patient: 64072843265,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/allergies")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&allergy_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_allergy(allergy_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = AllergyService::new(&client);

        let result = service.create(&allergy_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_allergy = result.unwrap();
        assert_eq!(created_allergy.id, allergy_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_allergy_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let allergy_id = 123456;
        let mock_allergy = Allergy {
            name: "Updated Erythromycin".to_owned(),
            ..get_mock_allergy(allergy_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/allergies/{}/", allergy_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "name": "Updated Erythromycin"
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_allergy).unwrap());
        });

        let client = Client::new().await.unwrap();
        let allergy_service = AllergyService::new(&client);

        let allergy_fu = AllergyForUpdate {
            name: Some("Updated Erythromycin".to_owned()),
            ..AllergyForUpdate::default()
        };

        let result = allergy_service.update(allergy_id, &allergy_fu).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let updated_allergy = result.unwrap();
        assert_eq!(updated_allergy.id, allergy_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_allergy_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let allergy_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/allergies/{}/", allergy_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let allergy_service = AllergyService::new(&client);

        let result = allergy_service.delete(allergy_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
