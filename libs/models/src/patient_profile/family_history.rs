use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::resource::Resource;

/// Represents a family history object in the patient profile.
///
/// Family history records detail the relationship of the family member, any associated medical conditions,
/// and include coding like SNOMED or ICD9.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FamilyHistory {
    /// The ID of the family history.
    pub id: i64,

    /// The relationship of the family member ("Mother", "Father", etc.).
    pub relationship: FamilyRelationship,

    /// The value or description of the family history.
    pub text: Option<String>,

    /// The ICD9 code associated with the family history (optional).
    pub icd9_code: Option<String>,

    /// The SNOMED code associated with the family history.
    pub snomed_code: Option<String>,

    /// The ID of the patient associated with the family history.
    pub patient: i64,

    /// The date the family history was created (read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<OffsetDateTime>,

    /// The date the family history was deleted (optional, read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents the relationship of the family member.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum FamilyRelationship {
    Mother,
    Father,
    Brother,
    Sister,
    Son,
    Daughter,
    Grandmother,
    Grandfather,
    Aunt,
    Uncle,
    Other,
}

/// Represents the data required to create a new family history.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FamilyHistoryForCreate {
    /// The relationship of the family member.
    pub relationship: FamilyRelationship,

    /// The value or description of the family history.
    pub text: Option<String>,

    /// The ICD9 code associated with the family history (optional).
    pub icd9_code: Option<String>,

    /// The SNOMED code associated with the family history.
    pub snomed_code: Option<String>,

    /// The ID of the patient associated with the family history.
    pub patient: i64,
}

/// Represents the data required to update an existing family history.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FamilyHistoryForUpdate {
    /// The relationship of the family member (optional).
    pub relationship: Option<FamilyRelationship>,

    /// The value or description of the family history (optional).
    pub text: Option<String>,

    /// The ICD9 code associated with the family history (optional).
    pub icd9_code: Option<String>,

    /// The SNOMED code associated with the family history (optional).
    pub snomed_code: Option<String>,
}

impl Resource for FamilyHistory {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/family_history"
    }
}
