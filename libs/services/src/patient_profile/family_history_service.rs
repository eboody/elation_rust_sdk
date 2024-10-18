use crate::base_service::BaseService;

use models::patient_profile::{FamilyHistory, FamilyHistoryForCreate, FamilyHistoryForUpdate};

pub type FamilyHistoryService<'a> =
    BaseService<'a, FamilyHistory, FamilyHistoryForCreate, FamilyHistoryForUpdate>;
