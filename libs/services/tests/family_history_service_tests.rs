#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, POST};
    use httpmock::MockServer;
    use models::patient_profile::{FamilyHistory, FamilyHistoryForCreate, FamilyRelationship};
    use serial_test::serial;
    use services::patient_profile::FamilyHistoryService;
    use services::prelude::*;
    use time::OffsetDateTime;

    fn get_mock_family_history(id: i64) -> FamilyHistory {
        FamilyHistory {
            id,
            relationship: FamilyRelationship::Mother,
            text: Some("History of hypertension".to_string()),
            icd9_code: None,
            snomed_code: Some("49436004".to_string()),
            patient: 64058687489,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_family_history_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let family_history_id = 123456;
        let family_history = get_mock_family_history(family_history_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/family_history/{}/", family_history_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&family_history).unwrap());
        });

        let client = Client::new().await.unwrap();
        let family_history_service = FamilyHistoryService::new(&client);

        let result = family_history_service.get(family_history_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let fetched_family_history = result.unwrap();
        assert_eq!(fetched_family_history.id, family_history_id);
        assert_eq!(
            fetched_family_history.relationship,
            FamilyRelationship::Mother
        );

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_family_history_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let family_history_id = 789012;
        let family_history_for_create = FamilyHistoryForCreate {
            relationship: FamilyRelationship::Mother,
            text: Some("History of hypertension".to_string()),
            icd9_code: None,
            snomed_code: Some("49436004".to_string()),
            patient: 64058687489,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/family_history")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&family_history_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_family_history(family_history_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = FamilyHistoryService::new(&client);

        let result = service.post(&family_history_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_family_history = result.unwrap();
        assert_eq!(created_family_history.id, family_history_id);

        mock.assert_async().await;
    }

    //#[serial]
    //#[tokio::test]
    //async fn test_update_patch_family_history_success() {
    //    let server = MockServer::start_async().await;
    //
    //    std::env::set_var("TEST_ENV", "TRUE");
    //    std::env::set_var("MOCK_SERVER_URL", server.base_url());
    //
    //    let family_history_id = 123456;
    //    let mock_family_history = FamilyHistory {
    //        text: Some("Updated history of hypertension".to_owned()),
    //        ..get_mock_family_history(family_history_id)
    //    };
    //
    //    let mock = server.mock(|when, then| {
    //        when.method(PATCH)
    //            .path(format!("/family_history/{}/", family_history_id))
    //            .header("Content-Type", "application/json")
    //            .json_body_partial(
    //                r#"{
    //                    "text": "Updated history of hypertension"
    //                }"#,
    //            );
    //        then.status(200)
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&mock_family_history).unwrap());
    //    });
    //
    //    let client = Client::new().await.unwrap();
    //    let family_history_service = FamilyHistoryService::new(&client);
    //
    //    let family_history_fu = FamilyHistoryForUpdate {
    //        text: Some("Updated history of hypertension".to_owned()),
    //        ..FamilyHistoryForUpdate::default()
    //    };
    //
    //    let result = family_history_service
    //        .patch(family_history_id, &family_history_fu)
    //        .await;
    //
    //    println!("result: {result:#?}");
    //
    //    assert!(result.is_ok());
    //    let updated_family_history = result.unwrap();
    //    assert_eq!(updated_family_history.id, family_history_id);
    //
    //    mock.assert_async().await;
    //}

    #[serial]
    #[tokio::test]
    async fn test_delete_family_history_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let family_history_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/family_history/{}/", family_history_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let family_history_service = FamilyHistoryService::new(&client);

        let result = family_history_service.delete(family_history_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
