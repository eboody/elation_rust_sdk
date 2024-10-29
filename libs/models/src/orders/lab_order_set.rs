use super::{FastingMethod, LabOrderTest, LabOrderTestForCreate, StatMethod};
use crate::{resource::Resource, Icd10Code};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};

time::serde::format_description!(one_true_date, Date, "[year]-[month]-[day]");

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderSet {
    /// The unique identifier of the lab order set.
    pub id: i64,
    /// The compendium code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compendium_code: Option<String>,
    /// The content of the lab order set.
    pub content: LabOrderSetContent,
    /// The date when the lab order set was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the lab order set was deleted (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The lab vendor ID.
    pub lab_vendor: i64,
    /// The name of the lab order set.
    pub name: String,
    /// The practice ID.
    pub practice: i64,
}

impl Resource for LabOrderSet {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/lab_order_sets"
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderSetContent {
    /// The unique identifier of the content.
    pub id: i64,
    /// The stat method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat_method: Option<StatMethod>,
    /// Notes for the patient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_instructions: Option<String>,
    /// Notes for the lab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_center_notes: Option<String>,
    /// Instructions for fasting before specimen collection.
    pub fasting_method: FastingMethod,
    /// Number of times the order should be performed on a standing basis.
    ///
    /// Can be up to 50 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing_order_frequency: Option<String>,

    /// Date at which the standing order should be canceled.
    #[serde(with = "one_true_date::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing_order_end_date: Option<Date>,

    /// If lab specimen collection is done onsite, this is the time at which the collection was taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_datetime: Option<OffsetDateTime>,

    /// ICD-10 diagnosis codes provided along with the order.
    pub icd10_codes: Vec<Icd10Code>,

    /// The list of tests that should be performed by the lab.
    ///
    /// Must all be either from the same compendium or no compendium.
    pub tests: Vec<LabOrderTest>,
}

/// Represents the data required to create a new lab order set.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderSetForCreate {
    /// The practice ID.
    ///
    /// Required.
    pub practice: i64,

    /// The name of the lab order set.
    ///
    /// Required.
    pub name: String,

    /// The content of the lab order set.
    ///
    /// Required.
    pub content: LabOrderSetContentForCreate,
}

/// Represents the content required to create a new lab order set.
///
/// Contains fields necessary for creating a lab order set, including the lab vendor ID.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderSetContentForCreate {
    /// The lab vendor ID.
    ///
    /// Required.
    pub lab_vendor: i64,

    /// The stat method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat_method: Option<StatMethod>,

    /// Notes for the patient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_instructions: Option<String>,

    /// Notes for the lab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_center_notes: Option<String>,

    /// Instructions for patient and lab on how long they should fast before specimen collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fasting_method: Option<FastingMethod>,

    /// Number of times the order should be performed on a standing basis.
    ///
    /// Can be up to 50 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing_order_frequency: Option<String>,

    /// Date at which the standing order should be canceled.
    #[serde(with = "one_true_date::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing_order_end_date: Option<Date>,

    /// If lab specimen collection is done onsite, this is the time at which the collection was taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_datetime: Option<OffsetDateTime>,

    /// ICD-10 diagnosis codes provided along with the order.
    #[serde(default)]
    pub icd10_codes: Vec<Icd10Code>,

    /// The list of tests that should be performed by the lab.
    ///
    /// Must all be either from the same compendium or no compendium.
    #[serde(default)]
    pub tests: Vec<LabOrderTestForCreate>,
}

/// Represents the data required to update an existing lab order set.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LabOrderSetForUpdate {
    /// The practice ID.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice: Option<i64>,

    /// The name of the lab order set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The content of the lab order set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<LabOrderSetContentForUpdate>,
}

/// Represents the content required to update an existing lab order set.
///
/// Contains fields necessary for updating a lab order set, including the lab vendor ID.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LabOrderSetContentForUpdate {
    /// The lab vendor ID.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lab_vendor: Option<i64>,

    /// The stat method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat_method: Option<StatMethod>,

    /// Notes for the patient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_instructions: Option<String>,

    /// Notes for the lab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_center_notes: Option<String>,

    /// Instructions for patient and lab on how long they should fast before specimen collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fasting_method: Option<FastingMethod>,

    /// Number of times the order should be performed on a standing basis.
    ///
    /// Can be up to 50 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing_order_frequency: Option<String>,

    /// Date at which the standing order should be canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing_order_end_date: Option<Date>,

    /// If lab specimen collection is done onsite, this is the time at which the collection was taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_datetime: Option<OffsetDateTime>,

    /// ICD-10 diagnosis codes provided along with the order.
    pub icd10_codes: Vec<Icd10Code>,

    /// The list of tests that should be performed by the lab.
    ///
    /// Must all be either from the same compendium or no compendium.
    pub tests: Vec<LabOrderTestForOrderSet>,
}

/// Represents a lab order test associated with a lab order set for creation or update.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderTestForOrderSet {
    /// The ID of the Lab Order Test.
    pub id: i64,
}
