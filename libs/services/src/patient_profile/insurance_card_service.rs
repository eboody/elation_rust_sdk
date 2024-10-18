use crate::base_service::BaseService;

use models::patient_profile::{InsuranceCard, InsuranceCardForCreate, InsuranceCardForUpdate};

pub type InsuranceCardService<'a> =
    BaseService<'a, InsuranceCard, InsuranceCardForCreate, InsuranceCardForUpdate>;
