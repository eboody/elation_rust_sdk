use crate::prelude::*;
use models::patient_profile::{Patient, PatientForCreate, PatientForUpdate, PatientQueryParams};

use crate::impl_service;

impl_service!(
    ServiceName: PatientService,
    Resource: Patient,
    ForCreate: PatientForCreate,
    ForUpdate: PatientForUpdate,
    QueryParams: PatientQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        PatchService,
        PutService,
        DeleteService
    ]
);
