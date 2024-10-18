use crate::base_service::BaseService;

use models::patient_profile::{PatientPhoto, PatientPhotoForCreate, PatientPhotoForUpdate};

pub type PatientPhotoService<'a> =
    BaseService<'a, PatientPhoto, PatientPhotoForCreate, PatientPhotoForUpdate>;
