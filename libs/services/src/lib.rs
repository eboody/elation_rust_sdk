pub mod base_service;
mod error;
mod patient_service;
//pub mod problem;
pub mod resource_service;

pub use error::{Error, Result};
pub use patient_service::*;
