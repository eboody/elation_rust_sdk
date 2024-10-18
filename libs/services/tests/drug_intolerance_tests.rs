#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::{
        DrugIntolerance, DrugIntoleranceForCreate, DrugIntoleranceForUpdate,
    };
    use patient_profile::DrugIntoleranceService;
    use resource_service::*;
    use serial_test::serial;
    use services::*;
    use time::{Date, OffsetDateTime};

    fn get_mock_drug_intolerance(id: i64) -> DrugIntolerance {
        DrugIntolerance {
            id,
            name: "Tylenol".to_string(),
            severity: Some("Mild".to_string()),
            reaction: Some("Hives".to_string()),
            patient: 140756523220993,
            start_date: Date::from_calendar_date(2016, time::Month::June, 24).unwrap(),
            status: true,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_drug_intolerance_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let intolerance_id = 123456;
        let intolerance = get_mock_drug_intolerance(intolerance_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/drug_intolerances/{}/", intolerance_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&intolerance).unwrap());
        });

        let client = Client::new().await.unwrap();
        let intolerance_service = DrugIntoleranceService::new(&client);

        let result = intolerance_service.get(intolerance_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let fetched_intolerance = result.unwrap();
        assert_eq!(fetched_intolerance.id, intolerance_id);
        assert_eq!(fetched_intolerance.name, "Tylenol");

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_drug_intolerance_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let intolerance_id = 789012;
        let intolerance_for_create = DrugIntoleranceForCreate {
            name: "Tylenol".to_string(),
            severity: Some("Mild".to_string()),
            reaction: Some("Hives".to_string()),
            patient: 140756523220993,
            start_date: Date::from_calendar_date(2016, time::Month::June, 24).unwrap(),
            status: true,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/drug_intolerances")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&intolerance_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_drug_intolerance(intolerance_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = DrugIntoleranceService::new(&client);

        let result = service.create(&intolerance_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_intolerance = result.unwrap();
        assert_eq!(created_intolerance.id, intolerance_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_drug_intolerance_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let intolerance_id = 123456;
        let mock_intolerance = DrugIntolerance {
            name: "Updated Tylenol".to_owned(),
            ..get_mock_drug_intolerance(intolerance_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/drug_intolerances/{}/", intolerance_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "name": "Updated Tylenol"
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_intolerance).unwrap());
        });

        let client = Client::new().await.unwrap();
        let intolerance_service = DrugIntoleranceService::new(&client);

        let intolerance_fu = DrugIntoleranceForUpdate {
            name: Some("Updated Tylenol".to_owned()),
            ..DrugIntoleranceForUpdate::default()
        };

        let result = intolerance_service
            .update(intolerance_id, &intolerance_fu)
            .await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let updated_intolerance = result.unwrap();
        assert_eq!(updated_intolerance.id, intolerance_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_drug_intolerance_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let intolerance_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/drug_intolerances/{}/", intolerance_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let intolerance_service = DrugIntoleranceService::new(&client);

        let result = intolerance_service.delete(intolerance_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
