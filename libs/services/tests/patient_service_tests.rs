#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{GET, POST};
    use httpmock::MockServer;
    use models::patient_profile::*;
    use services::{Error, PatientService};
    use time::{Date, OffsetDateTime};

    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_patient_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /patients/{id}/ endpoint
        let patient_id = 123456;
        let patient = get_mock_patient(patient_id);

        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("/patients/{}/", patient_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&patient).unwrap());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = PatientService::new(&client);

        // Call the method under test
        let result = service.get_patient(patient_id).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let patient = result.unwrap();
        assert_eq!(patient.id, patient_id);
        assert_eq!(patient.first_name, "John");
        assert_eq!(patient.last_name, "Doe");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_patient_success() {
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Create the patient data for the mock
        let patient_id = 789012;

        // Prepare the patient data to create
        let patient_for_create = PatientForCreate {
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            dob: Date::from_calendar_date(1985, time::Month::May, 5).unwrap(),
            sex: Sex::Female,
            primary_physician: 1,
            caregiver_practice: 2,
            address: None,
            emails: None,
            insurances: vec![],
        };

        // Mock the POST /patients/ endpoint
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/patients/")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&patient_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(patient_id.to_string());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = PatientService::new(&client);

        // Call the method under test
        let result = service.create_patient(&patient_for_create).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let updated_patient_id = result.unwrap();
        assert_eq!(updated_patient_id, 789012);

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_patient_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        // Mock the PUT /patients/{id}/ endpoint
        let patient_id = 123456;

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/patients/{}/", patient_id))
                .header("Content-Type", "application/json")
                .json_body_partial(
                    r#"{
                        "first_name": "Johnny"
                    }"#,
                );
            then.status(200)
                .header("Content-Type", "application/json")
                .body(patient_id.to_string());
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = PatientService::new(&client);

        let patient_fu = PatientForUpdate {
            first_name: Some("Johnny".to_owned()),
            ..PatientForUpdate::default()
        };

        // Call the method under test
        let result = service.update_patient(patient_id, &patient_fu).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let patient = result.unwrap();
        assert_eq!(patient, patient_id);
        //assert_eq!(patient.first_name, "John");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_patient_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the DELETE /patients/{id}/ endpoint
        let patient_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path(format!("/patients/{}/", patient_id));
            then.status(204); // No Content
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = PatientService::new(&client);

        // Call the method under test
        let result = service.delete_patient(patient_id).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_find_patients_success() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let mock_patient = get_mock_patient(123456);
        let mut mock_patient_two = get_mock_patient(78910);

        mock_patient_two.last_name = "Smith".to_owned();

        let vec_of_mock_patients =
            serde_json::to_string(&vec![mock_patient, mock_patient_two]).unwrap();

        // Mock the GET /patients/ endpoint with query parameters
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/patients/")
                .query_param("first_name", "John");

            then.status(200)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{ \"results\": {vec_of_mock_patients}, \"next\": null, \"previous\": null, \"count\": 2 }}"
                ));
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = PatientService::new(&client);

        // Prepare query parameters
        let query_params = PatientQueryParams {
            first_name: Some("John".to_string()),
            ..Default::default()
        };

        // Call the method under test
        let result = service.find_patients(query_params).await;

        println!("result: {result:#?}");

        // Assert the result
        assert!(result.is_ok());
        let patients = result.unwrap().results;
        assert_eq!(patients.len(), 2);
        assert_eq!(patients[0].first_name, "John");
        assert_eq!(patients[1].first_name, "John");

        // Ensure the mock was called
        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_get_patient_not_found() {
        // Start a local mock server
        let server = MockServer::start_async().await;

        // Set the mock env
        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        // Mock the GET /patients/{id}/ endpoint to return 404
        let patient_id = 999999;
        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("/patients/{}/", patient_id));
            then.status(404)
                .header("Content-Type", "application/json")
                .body(
                    r#"{
                        "detail": "Not found."
                    }"#,
                );
        });

        // Create a client pointing to the mock server
        let client = Client::new().await.unwrap();
        let service = PatientService::new(&client);

        // Call the method under test
        let result = service.get_patient(patient_id).await;

        // Assert the result
        assert!(result.is_err());
        let error = result.err().unwrap();

        // Match the error variant
        match error {
            Error::ClientError(client_error) => match client_error {
                client::Error::NotFound(_) => {
                    // Expected error
                }
                _ => panic!("Expected NotFound error"),
            },
            _ => panic!("Expected ClientError wrapping NotFound error"),
        }

        // Ensure the mock was called
        mock.assert_async().await;
    }
    fn get_mock_patient(patient_id: i64) -> Patient {
        Patient {
            id: patient_id,
            first_name: "John".to_string(),
            middle_name: Some("Middle".to_string()),
            last_name: "Doe".to_string(),
            actual_name: Some("Johnathan Doe".to_string()),
            gender_identity: Some(GenderIdentity::Man),
            legal_gender_marker: Some(LegalGenderMarker::M),
            pronouns: Some(Pronouns::HeHimHis),
            sex: Sex::Male,
            sexual_orientation: Some(SexualOrientation::Straight),
            primary_physician: 123,  // Example physician ID
            caregiver_practice: 456, // Example caregiver practice ID
            dob: Date::from_calendar_date(1990, time::Month::January, 1).unwrap(), // Example DOB
            ssn: Some("123-45-6789".to_string()), // Example SSN
            race: Some(Race::White),
            ethnicity: Some(Ethnicity::NotHispanicOrLatino),
            preferred_language: Some("English".to_string()),
            notes: Some("No known allergies.".to_string()),
            vip: false,
            tags: vec!["example_tag".to_string()],
            sms_opt_in_status: Some(true),
            address: Some(Address {
                address_line1: "123 Main St".to_string(),
                address_line2: None,
                city: Some("Example City".to_string()),
                state: Some("CA".to_string()),
                zip: Some("90210".to_string()),
                phones: vec![Phone {
                    phone: "555-1234".to_string(),
                    phone_type: "mobile".to_string(),
                }],
            }),
            phones: Some(vec![Phone {
                phone: "555-5678".to_string(),
                phone_type: "home".to_string(),
            }]),
            emails: Some(vec![Email {
                email: "john.doe@example.com".to_string(),
            }]),
            guarantor: Some(Guarantor {
                id: Some(789),
                address: Some("456 Elm St".to_string()),
                city: Some("Another City".to_string()),
                state: Some("NY".to_string()),
                zip: Some("10001".to_string()),
                phone: Some("555-8765".to_string()),
                email: Some("guarantor@example.com".to_string()),
                relationship: Some(GuarantorRelationship::Spouse),
                first_name: Some("Jane".to_string()),
                last_name: Some("Doe".to_string()),
                middle_name: None,
            }),
            insurances: Some(vec![Insurance {
                member_id: "INS123456".to_string(),
                rank: "Primary".to_string(),
            }]),
            deleted_insurances: None,
            preference: Some(Preference {
                preferred_pharmacy_1: Some("Pharmacy A".to_string()),
                preferred_pharmacy_2: Some("Pharmacy B".to_string()),
            }),
            emergency_contact: Some(EmergencyContact {
                first_name: Some("Jane".to_string()),
                last_name: Some("Doe".to_string()),
                relationship: Some(EmergencyContactRelationship::Spouse),
                phone: Some("555-8765".to_string()),
                address_line1: Some("123 Main St".to_string()),
                address_line2: None,
                city: Some("Example City".to_string()),
                state: Some("CA".to_string()),
                zip: Some("90210".to_string()),
            }),
            previous_name: None,
            master_patient: None,
            employer: Some(Employer {
                code: Some("EMP123".to_string()),
                name: Some("Example Employer".to_string()),
                description: Some("Description of employer".to_string()),
            }),
            consents: Some(vec![Consent {
                consented: true,
                last_modified_date: Some(OffsetDateTime::now_utc()),
                application: Some("App".to_string()),
            }]),
            metadata: None,
            merged_into_chart: None,
            primary_care_provider: Some(987),
            primary_care_provider_npi: Some("1234567890".to_string()),
            patient_status: PatientStatus {
                deceased_date: None,
                inactive_reason: None,
                last_status_change: Some("2024-01-01".to_string()),
                notes: Some("Active patient.".to_string()),
                status: PatientStatusEnum::Active,
            },
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
        }
    }
}
