#[cfg(test)]
mod tests {
    use client::Client;
    use httpmock::Method::{DELETE, GET, PATCH, POST};
    use httpmock::MockServer;
    use models::patient_profile::{
        Immunization, ImmunizationForCreate, ImmunizationForUpdate, Vaccine, VaccineForCreate,
    };
    use serial_test::serial;
    use services::*;
    use time::OffsetDateTime;

    fn get_mock_immunization(immunization_id: i64) -> Immunization {
        Immunization {
            id: immunization_id,
            administered_date: OffsetDateTime::now_utc(),
            administering_physician: 720898,
            ordering_physician: 720898,
            description: "Comvax Intramuscular Suspension".to_string(),
            reason: Some("Routine immunization".to_string()),
            vaccine: Vaccine {
                id: 1,
                description: Some("desc".to_owned()),
                ndc: None,
                ndc_values: None,
                practice: Some("1".to_owned()),
                name: Some("Hib-Hep B".to_string()),
                cvx: 51,
                cdc_name: Some("Haemophilus influenzae type b".to_string()),
                cdc_type: Some("Hib-HepB".to_string()),
                created_date: Some(OffsetDateTime::now_utc()),
                deleted_date: None,
            },
            qty: None,
            qty_units: Some("ml".to_string()),
            lot_number: Some("123456".to_string()),
            manufacturer_name: Some("Manufacturer X".to_string()),
            manufacturer_code: Some("MX".to_string()),
            expiration_date: None,
            vis: Some("VIS Document".to_string()),
            method: "Intramuscular".to_string(),
            site: "R Deltoid (RD)".to_string(),
            notes: Some("No adverse reaction.".to_string()),
            publicity_code: Some("Reminder/recall - any method".to_string()),
            vfc_eligibility: Some("VFC eligible".to_string()),
            funding_source: None,
            info_source: Some("New immunization record".to_string()),
            allowed_sharing: Some(true),
            practice: 140758835265540,
            patient: 140758844637185,
            created_date: Some(OffsetDateTime::now_utc()),
            deleted_date: None,
            dose_in_series: Some("1 of 3".to_string()),
            priority_population: None,
            ndc: None,
            patient_consent: Some(true),
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_get_immunization_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let immunization_id = 123456;
        let immunization = get_mock_immunization(immunization_id);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/immunizations/{}/", immunization_id));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&immunization).unwrap());
        });

        let client = Client::new().await.unwrap();
        let immunization_service = ImmunizationService::new(&client);

        let result = immunization_service.get(immunization_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let fetched_immunization = result.unwrap();
        assert_eq!(fetched_immunization.id, immunization_id);
        assert_eq!(
            fetched_immunization.vaccine.name,
            Some("Hib-Hep B".to_owned())
        );

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_create_immunization_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let immunization_id = 789012;
        let immunization_for_create = ImmunizationForCreate {
            administered_date: OffsetDateTime::now_utc(),
            administering_physician: 720898,
            ordering_physician: 720898,
            description: "Comvax Intramuscular Suspension".to_string(),
            vaccine: VaccineForCreate {
                description: "desc".to_owned(),
                ndc: true,
                ndc_values: None,
                practice: Some("1".to_owned()),
                name: "Hib-Hep B".to_string(),
                cvx: 51,
                cdc_name: Some("Haemophilus influenzae type b".to_string()),
                cdc_type: "Hib-HepB".to_string(),
            },
            method: "Intramuscular".to_string(),
            site: "R Deltoid (RD)".to_string(),
            patient: 140758844637185,
        };

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/immunizations")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&immunization_for_create).unwrap());
            then.status(201)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&get_mock_immunization(immunization_id)).unwrap());
        });

        let client = Client::new().await.unwrap();
        let immunization_service = ImmunizationService::new(&client);

        let result = immunization_service.create(&immunization_for_create).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let created_immunization = result.unwrap();
        assert_eq!(created_immunization.id, immunization_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_update_patch_immunization_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let immunization_id = 123456;
        let mock_immunization = Immunization {
            description: "Updated description".to_owned(),
            ..get_mock_immunization(immunization_id)
        };

        let mock = server.mock(|when, then| {
            when.method(PATCH)
                .path(format!("/immunizations/{}/", immunization_id))
                .header("Content-Type", "application/json")
                .json_body_partial(r#"{"description": "Updated description"}"#);
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&mock_immunization).unwrap());
        });

        let client = Client::new().await.unwrap();
        let immunization_service = ImmunizationService::new(&client);

        let immunization_fu = ImmunizationForUpdate {
            description: Some("Updated description".to_owned()),
            ..ImmunizationForUpdate::default()
        };

        let result = immunization_service
            .update(immunization_id, &immunization_fu)
            .await;

        println!("result: {result:#?}");

        assert!(result.is_ok());
        let updated_immunization = result.unwrap();
        assert_eq!(updated_immunization.id, immunization_id);

        mock.assert_async().await;
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_immunization_success() {
        let server = MockServer::start_async().await;

        std::env::set_var("TEST_ENV", "TRUE");
        std::env::set_var("MOCK_SERVER_URL", server.base_url());

        let immunization_id = 123456;
        let mock = server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("/immunizations/{}/", immunization_id));
            then.status(204);
        });

        let client = Client::new().await.unwrap();
        let immunization_service = ImmunizationService::new(&client);

        let result = immunization_service.delete(immunization_id).await;

        println!("result: {result:#?}");

        assert!(result.is_ok());

        mock.assert_async().await;
    }
}
