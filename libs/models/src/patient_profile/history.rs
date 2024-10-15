use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::resource::Resource;

/// Represents a patient's history entry.
///
/// The history object can represent various types of medical or personal history,
/// including dietary habits, family history, social history, and more.
/// It tracks the rank and the text value of the history entry, along with the patient ID.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    /// The ID of the history entry.
    pub id: i64,

    /// The type of the history entry (e.g., "Diet", "Past", "Social").
    pub r#type: HistoryType,

    /// The rank (or sequence) of the history entry.
    pub rank: i32,

    /// The text or value of the history entry.
    pub text: String,

    /// The ID of the patient associated with the history entry.
    pub patient: i64,

    /// The date the history entry was created (read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<OffsetDateTime>,

    /// The date the history entry was deleted (optional, read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents the type of a history entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum HistoryType {
    Past,
    Family,
    Social,
    Habits,
    Diet,
    Exercise,
    Immunization,
    Legal,
    Consultation,
    HealthMaintenance,
    PastSurgical,
    CognitiveStatus,
    FunctionalStatus,
}

/// Represents the data required to create a new history entry.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryForCreate {
    /// The type of the history entry.
    pub r#type: HistoryType,

    /// The rank (or sequence) of the history entry.
    pub rank: i32,

    /// The text or value of the history entry.
    pub text: String,

    /// The ID of the patient associated with the history entry.
    pub patient: i64,
}

/// Represents the data required to update an existing history entry.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct HistoryForUpdate {
    /// The type of the history entry (optional).
    pub r#type: Option<HistoryType>,

    /// The rank (or sequence) of the history entry (optional).
    pub rank: Option<i32>,

    /// The text or value of the history entry (optional).
    pub text: Option<String>,
}

impl Resource for History {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/histories"
    }
}
