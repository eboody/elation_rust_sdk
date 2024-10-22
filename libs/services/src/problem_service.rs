use crate::Result;
use crate::{base_service::BaseService, ResourceService};

use client::Client;
use models::patient_profile::{Problem, ProblemForCreate, ProblemForUpdate};

//pub type ProblemService<'a> = BaseService<'a, Problem, ProblemForCreate, ProblemForUpdate>;

//NOTE: If a specific service requires additional methods not covered by ResourceService,
//we can create a new struct that embeds BaseService and implements the additional methods.

pub struct ProblemService<'a> {
    base: BaseService<'a, Problem, ProblemForCreate, ProblemForUpdate>,
}

impl<'a> ProblemService<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            base: BaseService::new(client),
        }
    }

    pub async fn get(&self, id: i64) -> Result<Problem> {
        self.base.get(id).await
    }

    pub async fn create(&self, problem_fc: &ProblemForCreate) -> Result<Problem> {
        self.base.create(problem_fc).await
    }
    pub async fn update(&self, id: i64, problem_fu: &ProblemForUpdate) -> Result<Problem> {
        self.base.update(id, problem_fu).await
    }
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.base.delete(id).await
    }
}
