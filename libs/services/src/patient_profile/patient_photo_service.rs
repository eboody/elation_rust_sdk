use crate::prelude::*;

use models::patient_profile::{PatientPhoto, PatientPhotoForCreate, PatientPhotoForUpdate};

impl_service!(
    ServiceName: PatientPhotoService,
    Resource: PatientPhoto,
    ForCreate: PatientPhotoForCreate,
    ForUpdate: PatientPhotoForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [GetService, PostService, DeleteService]
);
