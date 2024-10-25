#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, POST};
    use httpmock::MockServer;
    use models::patient_profile::{AllergyDocumentation, AllergyDocumentationForCreate};
    use serial_test::serial;
    use services::patient_profile::AllergyDocumentationService;
    use services::prelude::*;
    use time::OffsetDateTime;

    fn get_mock_allergy_documentation(doc_id: i64) -> AllergyDocumentation {
        AllergyDocumentation {
            id: doc_id,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
            patient: 64072843265,
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_allergy_documentation_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let doc_id = 123456;
        let documentation = get_mock_allergy_documentation(doc_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/allergy_documentation/{}/", doc_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&documentation).unwrap());
        });

        let client = Client::new().await.unwrap();
        let documentation_service = AllergyDocumentationService::new(&client);

        let result = documentation_service.get(doc_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let fetched_documentation = result.unwrap();
        assert_eq!(fetched_documentation.id, doc_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_allergy_documentation_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let doc_id = 789012;
        let documentation_for_create = AllergyDocumentationForCreate {
            patient: 64072843265,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/allergy_documentation")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&documentation_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_allergy_documentation(doc_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = AllergyDocumentationService::new(&client);

        let result = service.post(&documentation_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_documentation = result.unwrap();
        assert_eq!(created_documentation.id, doc_id);

        mock.assert_async().await;
    }

    //#[serial]
    //#[tokio::test]
    //async fn test_update_patch_allergy_documentation_success() {
    //    let server = MockServer::start_async().await;
    //
    //    std::env::set_var("TEST_ENV", "TRUE");
    //    std::env::set_var("MOCK_SERVER_URL", server.base_url());
    //
    //    let doc_id = 123456;
    //    let mock_documentation = AllergyDocumentation {
    //        patient: 64072843265,
    //        ..get_mock_allergy_documentation(doc_id)
    //    };
    //
    //    let mock = server.mock(|when, then| {
    //        when.method(PATCH)
    //            .path(format!("/allergy_documentation/{}/", doc_id))
    //            .header("Content-Type", "application/json")
    //            .json_body_partial(r#"{"patient": 64072843265}"#);
    //        then.status(200)
    //            .header("Content-Type", "application/json")
    //            .body(serde_json::to_string(&mock_documentation).unwrap());
    //    });
    //
    //    let client = Client::new().await.unwrap();
    //    let documentation_service = AllergyDocumentationService::new(&client);
    //
    //    let documentation_fu = AllergyDocumentationForUpdate {
    //        patient: Some(64072843265),
    //    };
    //
    //    let result = documentation_service
    //        .update(doc_id, &documentation_fu)
    //        .await;
    //
    //    println!("result: {result:#?}");
    //
    //    assert!(result.is_ok());
    //    let updated_documentation = result.unwrap();
    //    assert_eq!(updated_documentation.id, doc_id);
    //
    //    mock.assert_async().await;
    //}

    #[serial]
    #[tokio::test]
    async fn test_delete_allergy_documentation_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let doc_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/allergy_documentation/{}/", doc_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let documentation_service = AllergyDocumentationService::new(&client);

        let result = documentation_service.delete(doc_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
