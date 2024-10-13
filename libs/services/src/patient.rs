use super::Result;
use client::{Client, PaginatedResponse};
use models::patient_profile::{Patient, PatientForCreate, PatientForUpdate, PatientQueryParams};

/// A service for interacting with patient-related operations in the Elation API.
///
/// The `PatientService` struct provides methods for performing CRUD operations
/// on patient resources, including creating, updating, deleting, and finding patients.
///
/// # Example
///
/// ```rust
/// use client::Client;
/// use services::PatientService;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new().await.unwrap();
///     let service = PatientService::new(&client);
///
///     let patient = service.get_patient(12345).await.unwrap();
///     println!("Patient: {:#?}", patient);
/// }
/// ```
pub struct PatientService<'a> {
    client: &'a Client,
}

impl<'a> PatientService<'a> {
    /// Creates a new instance of `PatientService`.
    ///
    /// This method initializes the service with a reference to the `Client`, which is used to
    /// make HTTP requests to the API.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the `Client` instance used for making API requests.
    ///
    /// # Returns
    ///
    /// A new instance of `PatientService`.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves a patient by ID.
    ///
    /// This method fetches a patient by their unique identifier from the Elation API.
    ///
    /// # Arguments
    ///
    /// * `patient_id` - The unique ID of the patient to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Patient` object if successful, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the patient is not found.
    pub async fn get_patient(&self, patient_id: i64) -> Result<Patient> {
        let endpoint = format!("/patients/{}/", patient_id);
        let response = self.client.get(&endpoint, ()).await?;
        let patient = response.json::<Patient>().await?;
        Ok(patient)
    }

    /// Creates a new patient.
    ///
    /// This method sends a request to create a new patient in the Elation API with the provided
    /// patient data.
    ///
    /// # Arguments
    ///
    /// * `patient` - A reference to a `PatientForCreate` struct containing the patient information.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the ID of the newly created patient if successful, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the patient creation is unsuccessful.
    pub async fn create_patient(&self, patient: &PatientForCreate) -> Result<i64> {
        let endpoint = "/patients/";
        let response = self.client.post(&endpoint, patient).await?;
        let new_patient = response.json::<i64>().await?;
        Ok(new_patient)
    }

    /// Updates an existing patient.
    ///
    /// This method sends a request to update an existing patient in the Elation API with the provided
    /// updated patient data.
    ///
    /// # Arguments
    ///
    /// * `patient_id` - The unique ID of the patient to update.
    /// * `patient` - A reference to a `PatientForUpdate` struct containing the updated patient information.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the ID of the updated patient if successful, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the update is unsuccessful.
    pub async fn update_patient(&self, patient_id: i64, patient: &PatientForUpdate) -> Result<i64> {
        let endpoint = format!("/patients/{}/", patient_id);
        let response = self.client.patch(&endpoint, patient).await?;
        let updated_patient = response.json::<i64>().await?;
        Ok(updated_patient)
    }

    pub async fn put_patient(&self, patient_id: i64, patient: &PatientForUpdate) -> Result<i64> {
        let endpoint = format!("/patients/{}/", patient_id);
        let response = self.client.put(&endpoint, patient).await?;
        let updated_patient = response.json::<i64>().await?;
        Ok(updated_patient)
    }

    /// Deletes a patient by ID.
    ///
    /// This method sends a request to delete an existing patient in the Elation API.
    ///
    /// # Arguments
    ///
    /// * `patient_id` - The unique ID of the patient to delete.
    ///
    /// # Returns
    ///
    /// Returns an empty `Result` if successful, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the deletion is unsuccessful.
    pub async fn delete_patient(&self, patient_id: i64) -> Result<()> {
        let endpoint = format!("/patients/{}/", patient_id);
        self.client.delete(&endpoint).await?;
        Ok(())
    }

    /// Finds patients based on query parameters.
    ///
    /// This method sends a request to find patients matching the provided query parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - A `PatientQueryParams` struct containing the query parameters for filtering patients.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `PaginatedResponse` of `Patient` objects if successful, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if no patients are found matching the query parameters.
    pub async fn find_patients(
        &self,
        params: PatientQueryParams,
    ) -> Result<PaginatedResponse<Patient>> {
        let endpoint = "/patients/";
        let response = self.client.get(&endpoint, params).await?;
        let paginated_response = response.json::<PaginatedResponse<Patient>>().await?;
        Ok(paginated_response)
    }
}
