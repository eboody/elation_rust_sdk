use crate::base_service::BaseService;

use models::patient_profile::{Problem, ProblemForCreate, ProblemForUpdate};

pub type ProblemService<'a> = BaseService<'a, Problem, ProblemForCreate, ProblemForUpdate>;

//NOTE: If a specific service requires additional methods not covered by ResourceService,
//we can create a new struct that embeds BaseService and implements the additional methods.

//pub struct ProblemService<'a> {
//    base: BaseService<'a, Problem, ProblemForCreate>,
//}
//
//impl<'a> ProblemService<'a> {
//    pub fn new(client: &'a Client) -> Self {
//        Self {
//            base: BaseService::new(client),
//        }
//    }
//
//    // Delegate methods to base
//    pub async fn get_problem(&self, id: i64) -> Result<Problem, Error> {
//        self.base.get(id).await
//    }
//
//    // Custom method
//    pub async fn custom_method(&self) -> Result<(), Error> {
//        // Custom implementation
//        Ok(())
//    }
//}
