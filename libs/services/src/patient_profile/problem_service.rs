use crate::prelude::*;
use models::patient_profile::{
    PatientProfileQueryParams, Problem, ProblemForCreate, ProblemForUpdate,
};

impl_service!(
    ServiceName: ProblemService,
    Resource: Problem,
    ForCreate: ProblemForCreate,
    ForUpdate: ProblemForUpdate,
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
