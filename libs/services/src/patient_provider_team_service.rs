use crate::base_service::BaseService;

use models::patient_profile::{
    PatientProviderTeam, PatientProviderTeamForCreate, PatientProviderTeamForUpdate,
};

pub type PatientProviderTeamService<'a> = BaseService<
    'a,
    PatientProviderTeam,
    PatientProviderTeamForCreate,
    PatientProviderTeamForUpdate,
>;
