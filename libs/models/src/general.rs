use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Icd10Code {
    /// The ICD-10 code.
    pub code: String,
    /// Description of the ICD-10 code.
    pub description: Option<String>,
}
