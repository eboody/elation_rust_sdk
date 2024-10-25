use crate::prelude::*;
use models::patient_profile::{
    Allergy, AllergyForCreate, AllergyForUpdate, PatientProfileQueryParams,
};

impl_service!(
    ServiceName: AllergyService,
    Resource: Allergy,
    ForCreate: AllergyForCreate,
    ForUpdate: AllergyForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [
        GetService,
        FindService,
        PostService,
        PutService,
        DeleteService
    ]
);
