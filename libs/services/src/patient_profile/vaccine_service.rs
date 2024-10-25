use crate::prelude::*;

use models::patient_profile::{
    PatientProfileQueryParams, Vaccine, VaccineForCreate, VaccineForUpdate,
};

impl_service!(
    ServiceName: VaccineService,
    Resource: Vaccine,
    ForCreate: VaccineForCreate,
    ForUpdate: VaccineForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [GetService, FindService, PostService, PatchService, DeleteService]
);
