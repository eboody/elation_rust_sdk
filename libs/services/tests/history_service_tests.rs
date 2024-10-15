#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::{History, HistoryForCreate, HistoryForUpdate, HistoryType};
    use serial_test::serial;
    use services::*;
    use time::OffsetDateTime;

    fn get_mock_history(id: i64) -> History {
        History {
            id,
            r#type: HistoryType::Diet,
            rank: 1,
            text: "Yogurt daily".to_string(),
            patient: 64072843265,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_history_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let history_id = 123456;
        let history = get_mock_history(history_id);

        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("/histories/{}/", history_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&history).unwrap());
        });

        let client = Client::new().await.unwrap();
        let history_service = HistoryService::new(&client);

        let result = history_service.get(history_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let fetched_history = result.unwrap();
        assert_eq!(fetched_history.id, history_id);
        assert_eq!(fetched_history.text, "Yogurt daily");

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_history_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let history_id = 789012;
        let history_for_create = HistoryForCreate {
            r#type: HistoryType::Diet,
            rank: 1,
            text: "Yogurt daily".to_string(),
            patient: 64072843265,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/histories")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&history_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_history(history_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = HistoryService::new(&client);

        let result = service.create(&history_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_history = result.unwrap();
        assert_eq!(created_history.id, history_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_history_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let history_id = 123456;
        let mock_history = History {
            text: "Updated Yogurt daily".to_owned(),
            ..get_mock_history(history_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/histories/{}/", history_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "text": "Updated Yogurt daily"
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_history).unwrap());
        });

        let client = Client::new().await.unwrap();
        let history_service = HistoryService::new(&client);

        let history_fu = HistoryForUpdate {
            text: Some("Updated Yogurt daily".to_owned()),
            ..HistoryForUpdate::default()
        };

        let result = history_service.update(history_id, &history_fu).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let updated_history = result.unwrap();
        assert_eq!(updated_history.id, history_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_history_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let history_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/histories/{}/", history_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let history_service = HistoryService::new(&client);

        let result = history_service.delete(history_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
