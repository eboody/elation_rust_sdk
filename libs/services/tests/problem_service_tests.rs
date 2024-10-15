#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::*;
    use serial_test::serial;
    use services::resource_service::ResourceService;
    use services::{Error, ProblemService};
    use time::Date;

    #[serial]
    #[tokio::test]
    async fn test_get_problem_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /problems/{id}/ endpoint
        let problem_id = 123456;
        let problem = get_mock_problem(problem_id);

        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("/problems/{}/", problem_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&problem).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let problem_service = ProblemService::new(&client);

        // Call the method under test
        let result = problem_service.get(problem_id).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let problem = result.unwrap();
        assert_eq!(problem.id, problem_id);
        assert_eq!(problem.description, "Chronic condition");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_problem_success() {
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Create the problem data for the mock
        let problem_id = 789012;

        // Prepare the problem data to create
        let problem_for_create = ProblemForCreate {
            description: "New health issue".to_string(),
            status: ProblemStatus::Active,
            synopsis: Some("Some synopsis".to_string()),
            start_date: Date::from_calendar_date(2023, time::Month::May, 5).unwrap(),
            resolved_date: None,
            dx: vec![],
            patient: 1,
        };

        // Mock the POST /problems/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/problems")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&problem_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_problem(problem_id)).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = ProblemService::new(&client);

        // Call the method under test
        let result = service.create(&problem_for_create).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let created_problem = result.unwrap();
        assert_eq!(created_problem.id, 789012);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_problem_success() {
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let problem_id = 123456;

        let mock_problem = Problem {
            description: "Updated description".to_owned(),
            ..get_mock_problem(problem_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/problems/{}/", problem_id))
                .header("Content-Type", "application/json")
                .json_body_partial(r#"{"description": "Updated description"}"#);
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_problem).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let problem_service = ProblemService::new(&client);

        let problem_fu = ProblemForUpdate {
            description: Some("Updated description".to_owned()),
            ..ProblemForUpdate::default()
        };

        // Call the method under test
        let result = problem_service.update(problem_id, &problem_fu).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let problem = result.unwrap();
        assert_eq!(problem.id, problem_id);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_problem_success() {
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let problem_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/problems/{}/", problem_id));
            then.status(204);
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let problem_service = ProblemService::new(&client);

        let result = problem_service.delete(problem_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_get_problem_not_found() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let problem_id = 999999;
        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("/problems/{}/", problem_id));
            then.status(404)
                .header("Content-Type", "application/json")
                .body(r#"{"detail": "Not found."}"#);
        });

        let client = Client::new().await.unwrap();
        let problem_service = ProblemService::new(&client);

        let result = problem_service.get(problem_id).await;

        assert!(result.is_err());
        let error = result.err().unwrap();

        match error {
            Error::ClientError(client_error) => match client_error {
                client::Error::NotFound(_) => {
                    // Expected error
                }
                _ => panic!("Expected NotFound error"),
            },
            _ => panic!("Expected ClientError wrapping NotFound error"),
        }

        mock.assert_async().await;
    }

    fn get_mock_problem(problem_id: i64) -> Problem {
        Problem {
            id: problem_id,
            description: "Chronic condition".to_string(),
            status: ProblemStatus::Active,
            synopsis: Some("Problem synopsis".to_string()),
            start_date: Date::from_calendar_date(2022, time::Month::January, 1).unwrap(),
            resolved_date: None,
            dx: vec![DxCode {
                icd9: Some(vec!["123.4".to_string()]),
                icd10: Some(vec!["A123".to_string()]),
                snomed: Some("SN123".to_string()),
            }],
            patient: 1,
            created_date: Some(time::OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }
}
