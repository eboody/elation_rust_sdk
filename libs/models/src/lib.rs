// Generated modules
pub mod billing;
pub mod care_gaps;
pub mod event_subscription;
pub mod insurance;
pub mod insurance_premium;
pub mod messaging;
pub mod orders;
pub mod patient_document;
pub mod patient_profile;
pub mod practice;
pub mod reference_data;
pub mod scheduling;
pub mod user_management;

pub mod resource;

// Re-export common models if desired
pub use patient_profile::Patient;
