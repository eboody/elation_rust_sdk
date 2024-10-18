use crate::base_service::BaseService;
pub use crate::resource_service::ResourceService;

use models::patient_profile::{
    AllergyDocumentation, AllergyDocumentationForCreate, AllergyDocumentationForUpdate,
};

pub type AllergyDocumentationService<'a> = BaseService<
    'a,
    AllergyDocumentation,
    AllergyDocumentationForCreate,
    AllergyDocumentationForUpdate,
>;
