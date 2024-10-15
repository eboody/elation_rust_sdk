use crate::base_service::BaseService;

use models::patient_profile::{
    DrugIntolerance, DrugIntoleranceForCreate, DrugIntoleranceForUpdate,
};

pub type DrugIntoleranceService<'a> =
    BaseService<'a, DrugIntolerance, DrugIntoleranceForCreate, DrugIntoleranceForUpdate>;
