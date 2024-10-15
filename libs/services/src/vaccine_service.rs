use crate::base_service::BaseService;

use models::patient_profile::{Vaccine, VaccineForCreate, VaccineForUpdate};

pub type VaccineService<'a> = BaseService<'a, Vaccine, VaccineForCreate, VaccineForUpdate>;
