use crate::base_service::BaseService;
use models::patient_profile::{Immunization, ImmunizationForCreate, ImmunizationForUpdate};

pub type ImmunizationService<'a> =
    BaseService<'a, Immunization, ImmunizationForCreate, ImmunizationForUpdate>;
