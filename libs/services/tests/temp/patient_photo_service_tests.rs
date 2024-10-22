#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::{PatientPhoto, PatientPhotoForCreate, PatientPhotoForUpdate};
    use patient_profile::PatientPhotoService;
    use serial_test::serial;
    use services::*;
    use time::OffsetDateTime;

    fn get_mock_patient_photo(photo_id: i64) -> PatientPhoto {
        PatientPhoto {
            id: photo_id,
            patient: 140754479349761,
            practice: 140754475089924,
            file: "https://feature-rm-28957-as-an-api-integrat.dev.elationemr.com/api/2.0/patients/140754479349761/get_photo".to_string(),
            content_type: "image/jpeg".to_string(),
            original_filename: "test_profile_image".to_string(),
            file_size: 122452,
            width: 961,
            height: 1280,
            last_updated: OffsetDateTime::now_utc(),
            created_date: OffsetDateTime::now_utc(),
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_patient_photo_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let photo_id = 123456;
        let patient_photo = get_mock_patient_photo(photo_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/patient_photos/{}/", photo_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&patient_photo).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let patient_photo_service = PatientPhotoService::new(&client);

        // Call the method under test
        let result = patient_photo_service.get(photo_id).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let fetched_photo = result.unwrap();
        assert_eq!(fetched_photo.id, photo_id);
        assert_eq!(fetched_photo.original_filename, "test_profile_image");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_patient_photo_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let photo_id = 789012;
        let photo_for_create = PatientPhotoForCreate {
            patient: 140754479349761,
            practice: 140754475089924,
            content_type: "image/jpeg".to_string(),
            original_filename: "test_profile_image".to_string(),
            file_size: 122452,
            width: 961,
            height: 1280,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/patient_photos")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&photo_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_patient_photo(photo_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = PatientPhotoService::new(&client);

        let result = service.create(&photo_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_photo = result.unwrap();
        assert_eq!(created_photo.id, photo_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_patient_photo_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let photo_id = 123456;
        let mock_photo = PatientPhoto {
            original_filename: "updated_profile_image".to_owned(),
            ..get_mock_patient_photo(photo_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/patient_photos/{}/", photo_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "original_filename": "updated_profile_image"
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_photo).unwrap());
        });

        let client = Client::new().await.unwrap();
        let patient_photo_service = PatientPhotoService::new(&client);

        let photo_fu = PatientPhotoForUpdate {
            original_filename: Some("updated_profile_image".to_owned()),
            ..Default::default()
        };

        let result = patient_photo_service.update(photo_id, &photo_fu).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let updated_photo = result.unwrap();
        assert_eq!(updated_photo.id, photo_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_patient_photo_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let photo_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/patient_photos/{}/", photo_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let patient_photo_service = PatientPhotoService::new(&client);

        let result = patient_photo_service.delete(photo_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
