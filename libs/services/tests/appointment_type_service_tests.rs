#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::{
        AppointmentType, AppointmentTypeForCreate, AppointmentTypeForUpdate,
    };
    use serial_test::serial;
    use services::*;
    use time::OffsetDateTime;

    fn get_mock_appointment_type(id: i64) -> AppointmentType {
        AppointmentType {
            abbreviation: Some("F/U".to_string()),
            color: None,
            default_duration: 15,
            description: Some("Follow-up visit".to_string()),
            patient_forms: vec![60471332291, 60471332292],
            patient_form_hours_prior: None,
            id,
            is_telehealth: false,
            name: "Follow-Up".to_string(),
            practice: 65540,
            patient_bookable: false,
            sequence: 0,
            visit_note_format: Some("twoCol.complex2".to_string()),
            visit_note_templates: vec![70681887140],
            visit_note_type: None,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_appointment_type_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let appointment_type_id = 123456;
        let appointment_type = get_mock_appointment_type(appointment_type_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/appointment_types/{}/", appointment_type_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&appointment_type).unwrap());
        });

        let client = Client::new().await.unwrap();
        let appointment_type_service = AppointmentTypeService::new(&client);

        let result = appointment_type_service.get(appointment_type_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let fetched_appointment_type = result.unwrap();
        assert_eq!(fetched_appointment_type.id, appointment_type_id);
        assert_eq!(fetched_appointment_type.name, "Follow-Up");

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_appointment_type_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let appointment_type_id = 789012;
        let appointment_type_for_create = AppointmentTypeForCreate {
            abbreviation: Some("F/U".to_string()),
            color: None,
            default_duration: 15,
            description: Some("Follow-up visit".to_string()),
            patient_forms: vec![60471332291, 60471332292],
            patient_form_hours_prior: None,
            is_telehealth: false,
            name: "Follow-Up".to_string(),
            practice: 65540,
            patient_bookable: false,
            sequence: 0,
            visit_note_format: Some("twoCol.complex2".to_string()),
            visit_note_templates: vec![70681887140],
            visit_note_type: None,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/appointment_types")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&appointment_type_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&get_mock_appointment_type(appointment_type_id)).unwrap(),
                );
        });

        let client = Client::new().await.unwrap();
        let service = AppointmentTypeService::new(&client);

        let result = service.create(&appointment_type_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_appointment_type = result.unwrap();
        assert_eq!(created_appointment_type.id, appointment_type_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_appointment_type_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let appointment_type_id = 123456;
        let mock_appointment_type = AppointmentType {
            name: "Updated Follow-Up".to_owned(),
            ..get_mock_appointment_type(appointment_type_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/appointment_types/{}/", appointment_type_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "name": "Updated Follow-Up"
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_appointment_type).unwrap());
        });

        let client = Client::new().await.unwrap();
        let appointment_type_service = AppointmentTypeService::new(&client);

        let appointment_type_fu = AppointmentTypeForUpdate {
            name: Some("Updated Follow-Up".to_owned()),
            ..AppointmentTypeForUpdate::default()
        };

        let result = appointment_type_service
            .update(appointment_type_id, &appointment_type_fu)
            .await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let updated_appointment_type = result.unwrap();
        assert_eq!(updated_appointment_type.id, appointment_type_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_appointment_type_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let appointment_type_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/appointment_types/{}/", appointment_type_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let appointment_type_service = AppointmentTypeService::new(&client);

        let result = appointment_type_service.delete(appointment_type_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
