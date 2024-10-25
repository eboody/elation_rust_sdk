use crate::prelude::*;

use models::patient_profile::{
    AppointmentType, AppointmentTypeForCreate, AppointmentTypeForUpdate, PatientProfileQueryParams,
};

impl_service!(
    ServiceName: AppointmentTypeService,
    Resource: AppointmentType,
    ForCreate: AppointmentTypeForCreate,
    ForUpdate: AppointmentTypeForUpdate,
    QueryParams: PatientProfileQueryParams,
    IdType: i64,
    Traits: [GetService, FindService, PatchService, DeleteService]
);
