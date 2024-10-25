use crate::prelude::*;
use models::patient_profile::{
    Immunization, ImmunizationForCreate, ImmunizationForUpdate, PatientProfileQueryParams,
};

impl_service!(
    ServiceName: ImmunizationService,
    Resource: Immunization,
    ForCreate: ImmunizationForCreate,
    ForUpdate: ImmunizationForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [GetService, FindService, PostService, DeleteService]
);
