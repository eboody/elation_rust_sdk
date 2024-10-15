use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::resource::Resource;

/// Represents the allergy documentation (NKDA) object.
///
/// The allergy documentation tracks whether a patient's allergies have been documented.
/// It helps determine if a patient has reported no allergies (NKDA), or if allergies have not been documented at all.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllergyDocumentation {
    /// The ID of the allergy documentation.
    pub id: i64,

    /// The date the documentation was created.
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<OffsetDateTime>,

    /// The date the documentation was deleted (optional).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<OffsetDateTime>,

    /// The ID of the patient associated with the allergy documentation.
    pub patient: i64,
}

/// Represents the data required to create new allergy documentation.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllergyDocumentationForCreate {
    /// The ID of the patient associated with the allergy documentation.
    pub patient: i64,
}

/// Represents the data required to update an existing allergy documentation.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AllergyDocumentationForUpdate {
    /// The ID of the patient associated with the allergy documentation (optional).
    pub patient: Option<i64>,
}

impl Resource for AllergyDocumentation {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/allergy_documentation"
    }
}
