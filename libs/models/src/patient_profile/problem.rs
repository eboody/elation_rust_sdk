use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::Date;

use crate::resource::Resource;

/// Represents a diagnosis in a patient's problem list.
///
/// Each problem can be potentially coded with ICD9, ICD10, SNOMED, and IMO.
/// `resolved_date` is set if status is "Resolved".
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    /// The ID of the problem.
    pub id: i64,

    /// The description of the problem.
    pub description: String,

    /// The status of the problem.
    pub status: ProblemStatus,

    /// The synopsis of the problem.
    pub synopsis: Option<String>,

    /// The date the problem started.
    pub start_date: Date,

    /// The date the problem was resolved (optional).
    pub resolved_date: Option<Date>,

    /// The diagnosis(es) for IMO codes.
    pub dx: Vec<DxCode>,

    /// The ID of the patient.
    pub patient: i64,

    /// The date the problem was created.
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<time::OffsetDateTime>,

    /// The date the problem was deleted (optional).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<time::OffsetDateTime>,
}

/// Represents the data required to create a new problem.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProblemForCreate {
    /// The description of the problem.
    pub description: String,

    /// The status of the problem.
    pub status: ProblemStatus,

    /// The synopsis of the problem.
    pub synopsis: Option<String>,

    /// The date the problem started.
    pub start_date: Date,

    /// The date the problem was resolved (optional).
    pub resolved_date: Option<Date>,

    /// The diagnosis(es) for IMO codes.
    pub dx: Vec<DxCode>,

    /// The ID of the patient.
    pub patient: i64,
}

/// Represents the status of a problem.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ProblemStatus {
    Active,
    Controlled,
    Resolved,
}

/// Represents diagnosis codes for a problem.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DxCode {
    /// The ICD9 codes of the diagnosis.
    pub icd9: Option<Vec<String>>,

    /// The ICD10 codes of the diagnosis.
    pub icd10: Option<Vec<String>>,

    /// The SNOMED code of the diagnosis.
    pub snomed: Option<String>,
}

/// Represents a problem update request for the Elation API.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProblemForUpdate {
    /// The status of the problem.
    pub status: Option<String>,

    /// A list of diagnosed objects for the problem.
    pub dx: Option<Vec<DxCode>>,

    /// The problem description.
    pub description: Option<String>,

    /// The rank or count of the problem.
    pub rank: Option<i32>,

    /// The date the problem was resolved.
    pub resolved_date: Option<Date>,

    /// A synopsis or details about the problem.
    pub synopsis: Option<String>,

    /// The date the problem started.
    pub start_date: Option<Date>,
}

/// Represents an ICD-10 code object.
#[derive(Debug, Serialize, Deserialize)]
pub struct Icd10Code {
    /// The ICD-10 code.
    pub code: String,
}

/// Represents query parameters for searching patients.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ProblemQueryParams {
    /// Vector of IDs of patients to find problems for
    pub patients: Vec<i64>,
    ///// Filter for last modified date greater than (optional).
    //pub last_modified__gt: Option<String>,
    ///// Filter for last modified date greater than or equal to (optional).
    //pub last_modified__gte: Option<String>,
    ///// Filter for last modified date less than (optional).
    //pub last_modified__lt: Option<String>,
    ///// Filter for last modified date less than or equal to (optional).
    //pub last_modified__lte: Option<String>,
    ///// The maximum number of results to return (optional).
    //pub limit: Option<i32>,
    ///// The offset for pagination (optional).
    //pub offset: Option<i32>,
}

impl Resource for Problem {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/problems"
    }
}
