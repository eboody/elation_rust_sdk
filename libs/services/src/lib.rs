mod allergy_documentation_service;
mod allergy_service;
mod appointment_type_service;
pub mod base_service;
mod drug_intolerance_service;
mod error;
mod family_history_service;
mod history;
mod immunzation_service;
mod insurance_card_service;
mod patient_photo_service;
mod patient_provider_team_service;
mod patient_service;
mod problem_service;
pub mod resource_service;
mod vaccine_service;

pub use allergy_documentation_service::*;
pub use allergy_service::*;
pub use appointment_type_service::*;
pub use drug_intolerance_service::*;
pub use error::{Error, Result};
pub use family_history_service::*;
pub use history::*;
pub use immunzation_service::*;
pub use insurance_card_service::*;
pub use patient_photo_service::*;
pub use patient_provider_team_service::*;
pub use patient_service::*;
pub use problem_service::*;
pub use vaccine_service::*;
