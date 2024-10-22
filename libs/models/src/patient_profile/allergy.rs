use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::Date;
use time::OffsetDateTime;

use crate::resource::Resource;

/// Represents an allergy object in the patient profile.
///
/// Each allergy can have a status (active or inactive), a start date, a reaction description,
/// and optional severity details. The Medi-Span identifiers are optional, and the patient ID
/// is associated with the allergy.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Allergy {
    /// The ID of the allergy.
    pub id: i64,

    /// The status of the allergy ("Active" or "Inactive").
    pub status: AllergyStatus,

    /// The date the allergy started.
    pub start_date: Date,

    /// The reaction to the drug.
    pub reaction: Option<String>,

    /// The name of the drug causing the allergy.
    pub name: String,

    /// The severity of the allergy.
    pub severity: Option<String>,

    /// The Medi-Span generic product code identifier.
    pub medispanid: Option<String>,

    /// The Medi-Span generic product code identifier.
    pub medispandnid: Option<String>,

    /// The ID of the patient associated with the allergy.
    pub patient: i64,

    /// The date the allergy was created.
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<OffsetDateTime>,

    /// The date the allergy was deleted (optional).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents the status of an allergy (either active or inactive).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AllergyStatus {
    Active,
    Inactive,
}

/// Represents the data required to create a new allergy.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllergyForCreate {
    /// The status of the allergy ("Active" or "Inactive").
    pub status: AllergyStatus,

    /// The date the allergy started.
    pub start_date: Date,

    /// The reaction to the drug (optional).
    pub reaction: Option<String>,

    /// The name of the drug causing the allergy.
    pub name: String,

    /// The severity of the allergy (optional).
    pub severity: Option<String>,

    /// The Medi-Span generic product code identifier (optional).
    pub medispanid: Option<String>,

    /// The Medi-Span generic product code identifier (optional).
    pub medispandnid: Option<String>,

    /// The ID of the patient associated with the allergy.
    pub patient: i64,
}

/// Represents the data required to update an existing allergy.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AllergyForUpdate {
    /// The status of the allergy ("Active" or "Inactive") (optional).
    pub status: Option<AllergyStatus>,

    /// The date the allergy started (optional).
    pub start_date: Option<Date>,

    /// The reaction to the drug (optional).
    pub reaction: Option<String>,

    /// The name of the drug causing the allergy (optional).
    pub name: Option<String>,

    /// The severity of the allergy (optional).
    pub severity: Option<String>,

    /// The Medi-Span generic product code identifier (optional).
    pub medispanid: Option<String>,

    /// The Medi-Span generic product code identifier (optional).
    pub medispandnid: Option<String>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AllergyQueryParams {
    pub patient_id: Option<Vec<i64>>,
}

impl Resource for Allergy {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/allergies"
    }
}
