use crate::prelude::*;

use models::patient_profile::{
    PatientProfileQueryParams, PatientProviderTeam, PatientProviderTeamForCreate,
    PatientProviderTeamForUpdate,
};

impl_service!(
    ServiceName: PatientProviderTeamService,
    Resource: PatientProviderTeam,
    ForCreate: PatientProviderTeamForCreate,
    ForUpdate: PatientProviderTeamForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        PatchService,
        DeleteService
    ]
);
