#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::{
        PatientProviderTeam, PatientProviderTeamForCreate, PatientProviderTeamForUpdate,
        PatientProviderTeamMember, PatientProviderTeamMemberForCreate,
        PatientProviderTeamMemberForUpdate,
    };
    use serial_test::serial;
    use services::*;
    use time::OffsetDateTime;

    fn get_mock_patient_provider_team(team_id: i64) -> PatientProviderTeam {
        PatientProviderTeam {
            patient_provider_team_id: team_id,
            patient_id: 237422977,
            team_members: vec![PatientProviderTeamMember {
                patient_provider_team_id: team_id,
                patient_provider_team_member_id: 140758911484332,
                patient_id: 237422977,
                physician_id: 483083,
                group: "main".to_string(),
                rank: 1,
                treatment_reason: Some("pcp at sacred heart".to_string()),
                earliest_activity: Some(OffsetDateTime::now_utc()),
                latest_activity: Some(OffsetDateTime::now_utc()),
                activity_summary_last_refreshed: None,
            }],
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_patient_provider_team_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let team_id = 123456;
        let patient_provider_team = get_mock_patient_provider_team(team_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/patient_provider_teams/{}/", team_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&patient_provider_team).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let patient_provider_team_service = PatientProviderTeamService::new(&client);

        // Call the method under test
        let result = patient_provider_team_service.get(team_id).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let fetched_team = result.unwrap();
        assert_eq!(fetched_team.patient_provider_team_id, team_id);
        assert_eq!(fetched_team.team_members[0].physician_id, 483083);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_patient_provider_team_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let team_id = 789012;
        let team_for_create = PatientProviderTeamForCreate {
            patient_id: 237422977,
            team_members: vec![PatientProviderTeamMemberForCreate {
                physician_id: 483083,
                group: "main".to_string(),
                rank: 1,
                treatment_reason: Some("pcp at sacred heart".to_string()),
            }],
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/patient_provider_teams")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&team_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_patient_provider_team(team_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let service = PatientProviderTeamService::new(&client);

        let result = service.create(&team_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_team = result.unwrap();
        assert_eq!(created_team.patient_provider_team_id, team_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_patient_provider_team_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let team_id = 123456;
        let mock_team = PatientProviderTeam {
            patient_provider_team_id: team_id,
            patient_id: 237422977,
            team_members: vec![PatientProviderTeamMember {
                patient_provider_team_id: team_id,
                patient_provider_team_member_id: 140758911484332,
                patient_id: 237422977,
                physician_id: 123456, // Updated physician ID
                group: "main".to_string(),
                rank: 1,
                treatment_reason: Some("Updated treatment reason".to_string()),
                earliest_activity: Some(OffsetDateTime::now_utc()),
                latest_activity: Some(OffsetDateTime::now_utc()),
                activity_summary_last_refreshed: None,
            }],
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/patient_provider_teams/{}/", team_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "team_members": [
                            {
                                "physician_id": 123456,
                                "treatment_reason": "Updated treatment reason"
                            }
                        ]
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_team).unwrap());
        });

        let client = Client::new().await.unwrap();
        let patient_provider_team_service = PatientProviderTeamService::new(&client);

        let team_fu = PatientProviderTeamForUpdate {
            team_members: Some(vec![PatientProviderTeamMemberForUpdate {
                physician_id: Some(123456),
                treatment_reason: Some("Updated treatment reason".to_string()),
                ..Default::default()
            }]),
        };

        let result = patient_provider_team_service
            .update(team_id, &team_fu)
            .await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let updated_team = result.unwrap();
        assert_eq!(updated_team.patient_provider_team_id, team_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_patient_provider_team_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let team_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/patient_provider_teams/{}/", team_id));
            then.status(204); // No Content
        });

        let client = Client::new().await.unwrap();
        let patient_provider_team_service = PatientProviderTeamService::new(&client);

        let result = patient_provider_team_service.delete(team_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
