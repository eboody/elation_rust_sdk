use crate::base_service::BaseService;

use models::patient_profile::{Allergy, AllergyForCreate, AllergyForUpdate};

pub type AllergyService<'a> = BaseService<'a, Allergy, AllergyForCreate, AllergyForUpdate>;
