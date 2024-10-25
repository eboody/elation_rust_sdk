use crate::prelude::*;

use models::patient_profile::{
    FamilyHistory, FamilyHistoryForCreate, FamilyHistoryForUpdate, PatientProfileQueryParams,
};

impl_service!(
    ServiceName: FamilyHistoryService,
    Resource: FamilyHistory,
    ForCreate: FamilyHistoryForCreate,
    ForUpdate: FamilyHistoryForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [GetService, FindService, PostService, DeleteService]
);
