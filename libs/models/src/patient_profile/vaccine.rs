use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::resource::Resource;

/// Represents a vaccine object, which can be a publicly accessible vaccine
/// from Medispan or Elation, or a vaccine entered by the practice.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vaccine {
    /// The ID of the vaccine.
    pub id: i64,

    /// The description of the vaccine.
    pub description: Option<String>,

    /// The name of the vaccine.
    pub name: Option<String>,

    /// The CVX (Vaccine Code) associated with the vaccine.
    pub cvx: i64,

    /// The CDC name of the vaccine.
    pub cdc_name: Option<String>,

    /// The CDC type of the vaccine.
    pub cdc_type: Option<String>,

    /// Indicates if the vaccine has an NDC (National Drug Code).
    pub ndc: Option<bool>,

    /// The NDC values associated with the vaccine (optional).
    pub ndc_values: Option<i64>,

    /// The ID of the practice associated with the vaccine (optional).
    pub practice: Option<String>,

    /// The date the vaccine was created (read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<OffsetDateTime>,

    /// The date the vaccine was deleted (optional, read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents the data required to create a new vaccine.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaccineForCreate {
    /// The description of the vaccine.
    pub description: String,

    /// The name of the vaccine.
    pub name: String,

    /// The CVX (Vaccine Code) associated with the vaccine.
    pub cvx: i64,

    /// The CDC name of the vaccine.
    pub cdc_name: Option<String>,

    /// The CDC type of the vaccine.
    pub cdc_type: String,

    /// Indicates if the vaccine has an NDC (National Drug Code).
    pub ndc: bool,

    /// The NDC values associated with the vaccine (optional).
    pub ndc_values: Option<i64>,

    /// The ID of the practice associated with the vaccine (optional).
    pub practice: Option<String>,
}

/// Represents the data required to update an existing vaccine.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VaccineForUpdate {
    /// The description of the vaccine (optional).
    pub description: Option<String>,

    /// The name of the vaccine (optional).
    pub name: Option<String>,

    /// The CVX (Vaccine Code) associated with the vaccine (optional).
    pub cvx: Option<i64>,

    /// The CDC name of the vaccine.
    pub cdc_name: Option<String>,

    /// The CDC type of the vaccine (optional).
    pub cdc_type: Option<String>,

    /// Indicates if the vaccine has an NDC (National Drug Code) (optional).
    pub ndc: Option<bool>,

    /// The NDC values associated with the vaccine (optional).
    pub ndc_values: Option<i64>,

    /// The ID of the practice associated with the vaccine (optional).
    pub practice: Option<String>,
}

impl Resource for Vaccine {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/vaccines"
    }
}
