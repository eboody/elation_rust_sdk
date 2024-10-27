use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};
use utils::time::Rfc3339;

use super::{pulmonary_order_test::PulmonaryOrderTest, Resolution};

use crate::resource::Resource;

time::serde::format_description!(one_true_date, Date, "[year]-[month]-[day]");

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PulmonaryOrder {
    /// The unique identifier of the pulmonary order.
    pub id: i64,
    /// Any allergies the patient has.
    pub allergies: Option<String>,
    /// The ID of the ancillary company the order is associated with.
    pub ancillary_company: i64,
    /// Array of Physician IDs to copy for the order.
    pub ccs: Option<Vec<i64>>,
    /// The chart date of the order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// The clinical reason for the order.
    pub clinical_reason: String,
    /// Whether the order is confidential.
    pub confidential: bool,
    /// The date when the pulmonary order was created (ISO 8601 format).
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the pulmonary order was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The document date of the order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// How to notify the patient of the results.
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    pub icd10_codes: Option<Vec<ICD10Code>>,
    /// The patient ID associated with the order.
    pub patient: i64,
    /// The practice ID associated with the order.
    pub practice: i64,
    /// The user ID associated with the order.
    pub prescribing_user: i64,
    /// The date when the order was signed.
    #[serde_as(as = "Option<Rfc3339>")]
    pub signed_date: Option<OffsetDateTime>,
    /// The user ID who signed the order.
    pub signed_by: Option<i64>,
    /// The resolution state of the order.
    pub resolution: Option<Resolution>,
    /// The pulmonary center ID associated with the order (nullable).
    pub pulmonary_center: Option<i64>,
    /// The date on which the tests should be performed, or were performed.
    #[serde(with = "one_true_date::option")]
    pub test_date: Option<Date>,
    /// An array of pulmonary order tests associated with the order.
    pub tests: Vec<PulmonaryOrderTest>,
}

impl Resource for PulmonaryOrder {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/pulmonary_orders"
    }
}

/// Represents an ICD-10 diagnosis code.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ICD10Code {
    /// The ICD-10 code.
    pub code: String,
    /// The description of the ICD-10 code.
    pub description: Option<String>,
}

///// Represents the resolution state of an order.
//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Resolution {
//    /// The unique identifier of the resolution.
//    pub id: i64,
//    /// The document ID associated with the resolution.
//    pub document: i64,
//    /// The resolving document ID (usually a report).
//    pub resolving_document: Option<i64>,
//    /// The state of the resolution.
//    pub state: ResolutionState,
//    /// Any note added to represent why an order was cancelled.
//    pub note: Option<String>,
//    /// The date when the resolution state was created.
//    #[serde_as(as = "Option<Rfc3339>")]
//    pub created_date: Option<OffsetDateTime>,
//    /// The date when the resolution was deleted.
//    #[serde_as(as = "Option<Rfc3339>")]
//    pub deleted_date: Option<OffsetDateTime>,
//}
//
///// Represents the possible states of a resolution.
//#[derive(Clone, Debug, Serialize, Deserialize)]
//#[serde(rename_all = "lowercase")]
//pub enum ResolutionState {
//    Outstanding,
//    Fulfilled,
//    Cancelled,
//}

/// Represents the data required to create a new pulmonary order.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PulmonaryOrderForCreate {
    /// Any allergies the patient has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergies: Option<String>,
    /// The ID of the ancillary company the order is associated with.
    pub ancillary_company: i64,
    /// Array of Physician IDs to copy for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccs: Option<Vec<i64>>,
    /// The chart date of the order.
    #[serde(with = "one_true_date")]
    pub chart_date: Date,
    /// The clinical reason for the order.
    pub clinical_reason: String,
    /// Whether the order is confidential.
    #[serde(default)]
    pub confidential: bool,
    /// The document date of the order.
    #[serde(with = "one_true_date")]
    pub document_date: Date,
    /// How to notify the patient of the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd10_codes: Option<Vec<ICD10Code>>,
    /// The patient ID associated with the order.
    pub patient: i64,
    /// The practice ID associated with the order.
    pub practice: i64,
    /// The user ID associated with the order.
    pub prescribing_user: i64,
    /// The pulmonary center ID associated with the order (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulmonary_center: Option<i64>,
    /// The tests associated with the order.
    pub tests: Vec<PulmonaryOrderTestForOrder>,
}

/// Represents the data required to update an existing pulmonary order.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PulmonaryOrderForUpdate {
    /// Any allergies the patient has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergies: Option<String>,
    /// The ID of the ancillary company the order is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_company: Option<i64>,
    /// Array of Physician IDs to copy for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccs: Option<Vec<i64>>,
    /// The chart date of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "one_true_date::option")]
    pub chart_date: Option<Date>,
    /// The clinical reason for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_reason: Option<String>,
    /// Whether the order is confidential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    /// The document date of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "one_true_date::option")]
    pub document_date: Option<Date>,
    /// How to notify the patient of the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd10_codes: Option<Vec<ICD10Code>>,
    /// The user ID associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescribing_user: Option<i64>,
    /// The pulmonary center ID associated with the order (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulmonary_center: Option<i64>,
    /// The tests associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<Vec<PulmonaryOrderTestForOrder>>,
}

/// Represents query parameters for searching pulmonary orders.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PulmonaryOrderQueryParams {
    /// The patient the order belongs to (optional).
    pub patient: Option<i64>,
    /// The practice the order belongs to (optional).
    pub practice: Option<i64>,
}

/// Represents a pulmonary order test associated with an order.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PulmonaryOrderTestForOrder {
    /// The ID of the pulmonary order test.
    pub id: i64,
}
