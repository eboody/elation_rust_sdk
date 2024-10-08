use super::Error;
use client::Client;
use models::patient_profile::{Patient, PatientForCreate, PatientQueryParams};

pub struct PatientService<'a> {
    client: &'a Client,
}

impl<'a> PatientService<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves a patient by ID.
    pub async fn get_patient(&self, patient_id: i64) -> Result<Patient, Error> {
        let endpoint = format!("/patients/{}/", patient_id);
        let response = self.client.get(&endpoint).await?;
        let patient = response.json::<Patient>().await?;
        Ok(patient)
    }

    /// Creates a new patient.
    pub async fn create_patient(&self, patient: &PatientForCreate) -> Result<Patient, Error> {
        let endpoint = "/patients/";
        let response = self.client.post(&endpoint, patient).await?;
        let new_patient = response.json::<Patient>().await?;
        Ok(new_patient)
    }

    /// Updates an existing patient.
    pub async fn update_patient(
        &self,
        patient_id: i64,
        patient: &Patient,
    ) -> Result<Patient, Error> {
        let endpoint = format!("/patients/{}/", patient_id);
        let response = self.client.put(&endpoint, patient).await?;
        let updated_patient = response.json::<Patient>().await?;
        Ok(updated_patient)
    }

    /// Deletes a patient by ID.
    pub async fn delete_patient(&self, patient_id: i64) -> Result<(), Error> {
        let endpoint = format!("/patients/{}/", patient_id);
        self.client.delete(&endpoint, patient_id).await?;
        Ok(())
    }

    /// Finds patients based on query parameters.
    pub async fn find_patients(&self, params: &PatientQueryParams) -> Result<Vec<Patient>, Error> {
        let endpoint = "/patients/";
        let response = self.client.get_with_params(&endpoint, params).await?;
        let paginated_response = response.json::<PaginatedResponse<Patient>>().await?;
        Ok(paginated_response.results)
    }
}
