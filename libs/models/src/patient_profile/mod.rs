mod allergy;
mod allergy_documentation;
mod appointment_type;
mod drug_intolerance;
mod family_history;
mod history;
mod immunization;
mod insurance_card;
mod patient;
mod patient_photo;
mod patient_provider_team;
mod problem;
mod vaccine;

pub use allergy::*;
pub use allergy_documentation::*;
pub use appointment_type::*;
pub use drug_intolerance::*;
pub use family_history::*;
pub use history::*;
pub use immunization::*;
pub use insurance_card::*;
pub use patient::*;
pub use patient_photo::*;
pub use patient_provider_team::*;
pub use problem::*;
pub use vaccine::*;

pub mod utils;

/// Represents the default query parameters for finding the various Patient Profile resources
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PatientProfileQueryParams {
    /// A vector of patient IDs
    pub patients: Vec<i64>,
}
