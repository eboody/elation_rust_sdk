use crate::prelude::*;

use models::patient_profile::{
    AllergyDocumentation, AllergyDocumentationForCreate, AllergyDocumentationForUpdate,
    PatientProfileQueryParams,
};

impl_service!(
    ServiceName: AllergyDocumentationService,
    Resource: AllergyDocumentation,
    ForCreate: AllergyDocumentationForCreate,
    ForUpdate: AllergyDocumentationForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [GetService, PostService, FindService, DeleteService]
);
