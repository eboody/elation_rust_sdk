use crate::prelude::*;
use models::patient_profile::{
    History, HistoryForCreate, HistoryForUpdate, PatientProfileQueryParams,
};

impl_service!(
    ServiceName: HistoryService,
    Resource: History,
    ForCreate: HistoryForCreate,
    ForUpdate: HistoryForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [GetService, FindService, PostService, DeleteService]
);
