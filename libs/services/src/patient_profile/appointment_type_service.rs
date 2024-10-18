use crate::base_service::BaseService;

use models::patient_profile::{
    AppointmentType, AppointmentTypeForCreate, AppointmentTypeForUpdate,
};

pub type AppointmentTypeService<'a> =
    BaseService<'a, AppointmentType, AppointmentTypeForCreate, AppointmentTypeForUpdate>;
