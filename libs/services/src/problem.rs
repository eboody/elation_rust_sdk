// libs/services/src/problem_service.rs

use crate::error::Error;
use client::{Client, PaginatedResponse, Params};
use models::patient_profile::{Problem, ProblemForCreate, ProblemStatus};
use serde::{Deserialize, Serialize};

pub struct ProblemService<'a> {
    client: &'a Client,
}

impl<'a> ProblemService<'a> {
    /// Creates a new instance of `ProblemService`.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves a problem by its ID.
    pub async fn get_problem(&self, problem_id: i64) -> Result<Problem, Error> {
        let endpoint = format!("/problems/{}/", problem_id);
        let response = self.client.get(&endpoint, ()).await?;
        let problem = response.json::<Problem>().await?;
        Ok(problem)
    }

    /// Creates a new problem.
    pub async fn create_problem(&self, problem: &ProblemForCreate) -> Result<Problem, Error> {
        let endpoint = "/problems/";
        let response = self.client.post(&endpoint, (), problem).await?;
        let new_problem = response.json::<Problem>().await?;
        Ok(new_problem)
    }

    /// Updates an existing problem.
    pub async fn update_problem(
        &self,
        problem_id: i64,
        problem: &ProblemForCreate,
    ) -> Result<Problem, Error> {
        let endpoint = format!("/problems/{}/", problem_id);
        let response = self.client.put(&endpoint, (), problem).await?;
        let updated_problem = response.json::<Problem>().await?;
        Ok(updated_problem)
    }

    /// Deletes a problem by its ID.
    pub async fn delete_problem(&self, problem_id: i64) -> Result<(), Error> {
        let endpoint = format!("/problems/{}/", problem_id);
        self.client.delete(&endpoint, ()).await?;
        Ok(())
    }

    /// Lists problems for a given patient.
    pub async fn list_problems(&self, patient_id: i64) -> Result<Vec<Problem>, Error> {
        let endpoint = "/problems/";
        let params = ListProblemsParams {
            patient: Some(patient_id),
            ..Default::default()
        };
        let response = self.client.get(&endpoint, params).await?;
        let paginated_response = response.json::<PaginatedResponse<Problem>>().await?;
        Ok(paginated_response.results)
    }
}

/// Query parameters for listing problems.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ListProblemsParams {
    pub patient: Option<i64>,
    pub status: Option<ProblemStatus>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Params for ListProblemsParams {}
