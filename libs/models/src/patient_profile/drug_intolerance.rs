use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};

use crate::resource::Resource;

/// Represents a drug intolerance in a patient's profile.
///
/// Each drug intolerance can have a name, severity, and reaction type, and is associated
/// with a patient. It tracks the start date, status (active or not), and metadata such as
/// creation and deletion dates.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrugIntolerance {
    /// The ID of the drug intolerance.
    pub id: i64,

    /// The name of the drug that the patient cannot tolerate.
    pub name: String,

    /// The severity of the patient's reaction (optional).
    pub severity: Option<String>,

    /// The reaction type experienced by the patient (optional).
    pub reaction: Option<String>,

    /// The ID of the patient associated with the drug intolerance.
    pub patient: i64,

    /// The date the drug intolerance started.
    pub start_date: Date,

    /// The status of the drug intolerance (active or inactive).
    pub status: bool,

    /// The date the drug intolerance was created.
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<OffsetDateTime>,

    /// The date the drug intolerance was deleted (optional).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents the data required to create a new drug intolerance.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrugIntoleranceForCreate {
    /// The name of the drug that the patient cannot tolerate.
    pub name: String,

    /// The severity of the patient's reaction (optional).
    pub severity: Option<String>,

    /// The reaction type experienced by the patient (optional).
    pub reaction: Option<String>,

    /// The ID of the patient associated with the drug intolerance.
    pub patient: i64,

    /// The date the drug intolerance started.
    pub start_date: Date,

    /// The status of the drug intolerance (active or inactive).
    pub status: bool,
}

/// Represents the data required to update an existing drug intolerance.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DrugIntoleranceForUpdate {
    /// The name of the drug that the patient cannot tolerate (optional).
    pub name: Option<String>,

    /// The severity of the patient's reaction (optional).
    pub severity: Option<String>,

    /// The reaction type experienced by the patient (optional).
    pub reaction: Option<String>,

    /// The date the drug intolerance started (optional).
    pub start_date: Option<Date>,

    /// The status of the drug intolerance (active or inactive) (optional).
    pub status: Option<bool>,
}

impl Resource for DrugIntolerance {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/drug_intolerances"
    }
}
