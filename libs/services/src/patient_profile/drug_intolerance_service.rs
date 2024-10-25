use crate::prelude::*;

use models::patient_profile::{DrugIntolerance, PatientProfileQueryParams};

impl_service!(
    ServiceName: DrugIntoleranceService,
    Resource: DrugIntolerance,
    ForCreate: (),
    ForUpdate: (),
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [GetService, FindService, DeleteService]
);
