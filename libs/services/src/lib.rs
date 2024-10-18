pub mod base_service;
pub mod macros;
pub mod resource_service;

pub mod orders;
pub mod patient_profile;

mod error;
pub use crate::resource_service::ResourceService;
pub use error::*;
