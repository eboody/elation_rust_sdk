pub mod base_service;
pub mod macros;
pub mod prelude;
pub mod resource_service;

pub mod orders;
pub mod patient_profile;

mod error;
pub use crate::resource_service::ResourceService;
pub use error::*;

#[macro_use]
extern crate doc_comment;
