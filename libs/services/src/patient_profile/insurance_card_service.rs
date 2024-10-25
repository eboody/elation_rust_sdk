use crate::prelude::*;

use models::patient_profile::{
    InsuranceCard, InsuranceCardForCreate, InsuranceCardForUpdate, PatientProfileQueryParams,
};

impl_service!(
    ServiceName: InsuranceCardService,
    Resource: InsuranceCard,
    ForCreate: InsuranceCardForCreate,
    ForUpdate: InsuranceCardForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i32,
    Traits: [ FindService, PostService, DeleteService ]
);
