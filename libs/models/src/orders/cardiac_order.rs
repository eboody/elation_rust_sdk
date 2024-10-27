use super::{CardiacOrderTest, Icd10Code, Resolution};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};
use utils::time::Rfc3339;

use crate::resource::Resource;

time::serde::format_description!(one_true_date, Date, "[year]-[month]-[day]");

///// Represents an ICD-10 code associated with a cardiac order.
//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Icd10Code {
//    /// The ICD-10 code.
//    pub code: String,
//    /// The description of the ICD-10 code.
//    pub description: String,
//}

//#[serde_as]
///// Represents the resolution state of a cardiac order.
//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Resolution {
//    /// Internal ID to represent the resolution state.
//    pub id: i64,
//    /// Will be the same as the ID of the cardiac order.
//    pub document: i64,
//    /// If the order is fulfilled, will indicate the document referenced.
//    pub resolving_document: Option<i64>,
//    /// The resolution state of the order.
//    pub state: ResolutionState,
//    /// Any note that was added to represent why an order was cancelled (optional).
//    pub note: Option<String>,
//    /// The datetime when this resolution state was created.
//    #[serde_as(as = "Option<Rfc3339>")]
//    pub created_date: Option<OffsetDateTime>,
//    /// The datetime when this resolution was deleted. Should never be null because we're providing the most updated resolution.
//    #[serde_as(as = "Option<Rfc3339>")]
//    pub deleted_date: Option<OffsetDateTime>,
//}

///// Represents the possible resolution states of a cardiac order.
//#[derive(Clone, Debug, Serialize, Deserialize)]
//#[serde(rename_all = "lowercase")]
//pub enum ResolutionState {
//    /// The order is outstanding.
//    Outstanding,
//    /// The order is fulfilled.
//    Fulfilled,
//    /// The order is cancelled.
//    Cancelled,
//}

/// Represents a cardiac order with detailed information.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardiacOrder {
    /// The unique identifier of the cardiac order.
    pub id: i64,
    /// The ancillary company the order is associated with.
    pub ancillary_company: i64,
    /// The cardiac center the order is associated with (nullable).
    pub cardiac_center: Option<i64>,
    /// Array of physician IDs to copy for the order.
    pub ccs: Vec<i64>,
    /// The chart date of the cardiac order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// The clinical reason for the order.
    pub clinical_reason: String,
    /// Whether the order is confidential.
    pub confidential: bool,
    /// The date when the cardiac order was created.
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the cardiac order was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The document date of the cardiac order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// How to notify the patient of the results.
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    pub icd10_codes: Vec<Icd10Code>,
    /// Description of beta blockers/medications the patient is taking.
    pub medications: String,
    /// The patient the order is associated with.
    pub patient: i64,
    /// The practice the order is associated with.
    pub practice: i64,
    /// The user associated with the order.
    pub prescribing_user: i64,
    /// The date when the order was signed.
    #[serde_as(as = "Option<Rfc3339>")]
    pub signed_date: Option<OffsetDateTime>,
    /// The ID of the user who signed the order.
    pub signed_by: Option<i64>,
    /// The resolution state of the cardiac order.
    pub resolution: Option<Resolution>,
    /// The date on which the tests should be performed, or were performed (nullable).
    #[serde(with = "one_true_date::option")]
    pub test_date: Option<Date>,
    /// A list of cardiac tests associated with the order.
    pub tests: Vec<CardiacOrderTest>,
}

impl Resource for CardiacOrder {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/cardiac_orders"
    }
}

/// Represents the data required to create a new cardiac order.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardiacOrderForCreate {
    /// The ancillary company the order is associated with.
    pub ancillary_company: i64,
    /// The cardiac center the order is associated with (optional).
    pub cardiac_center: Option<i64>,
    /// Array of physician IDs to copy for the order.
    pub ccs: Option<Vec<i64>>,
    /// The chart date of the cardiac order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// The clinical reason for the order.
    pub clinical_reason: String,
    /// Whether the order is confidential.
    pub confidential: Option<bool>,
    /// The document date of the cardiac order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// How to notify the patient of the results.
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    pub icd10_codes: Option<Vec<Icd10Code>>,
    /// Description of beta blockers/medications the patient is taking.
    pub medications: Option<String>,
    /// The patient the order is associated with.
    pub patient: i64,
    /// The practice the order is associated with.
    pub practice: i64,
    /// The user associated with the order.
    pub prescribing_user: i64,
    /// A list of cardiac tests associated with the order.
    pub tests: Option<Vec<CardiacOrderTest>>,
    /// The date on which the tests should be performed, or were performed (optional).
    #[serde(with = "one_true_date::option")]
    pub test_date: Option<Date>,
}

/// Represents the data required to update a cardiac order (PUT).
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardiacOrderForUpdate {
    /// The ancillary company the order is associated with.
    pub ancillary_company: i64,
    /// The cardiac center the order is associated with (optional).
    pub cardiac_center: Option<i64>,
    /// Array of physician IDs to copy for the order.
    pub ccs: Option<Vec<i64>>,
    /// The chart date of the cardiac order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// The clinical reason for the order.
    pub clinical_reason: String,
    /// Whether the order is confidential.
    pub confidential: Option<bool>,
    /// The document date of the cardiac order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// How to notify the patient of the results.
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    pub icd10_codes: Option<Vec<Icd10Code>>,
    /// Description of beta blockers/medications the patient is taking.
    pub medications: Option<String>,
    /// The patient the order is associated with (cannot be changed on update).
    pub patient: i64,
    /// The practice the order is associated with (cannot be changed on update).
    pub practice: i64,
    /// The user associated with the order.
    pub prescribing_user: i64,
    /// A list of cardiac tests associated with the order.
    pub tests: Option<Vec<CardiacOrderTest>>,
    /// The date on which the tests should be performed, or were performed (optional).
    #[serde(with = "one_true_date::option")]
    pub test_date: Option<Date>,
}

/// Represents the data required to partially update a cardiac order (PATCH).
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CardiacOrderForPatch {
    /// The ancillary company the order is associated with.
    pub ancillary_company: Option<i64>,
    /// The cardiac center the order is associated with (optional).
    pub cardiac_center: Option<i64>,
    /// Array of physician IDs to copy for the order.
    pub ccs: Option<Vec<i64>>,
    /// The chart date of the cardiac order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// The clinical reason for the order.
    pub clinical_reason: Option<String>,
    /// Whether the order is confidential.
    pub confidential: Option<bool>,
    /// The document date of the cardiac order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// How to notify the patient of the results.
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    pub icd10_codes: Option<Vec<Icd10Code>>,
    /// Description of beta blockers/medications the patient is taking.
    pub medications: Option<String>,
    /// The patient the order is associated with (cannot be changed on update).
    pub patient: Option<i64>,
    /// The practice the order is associated with (cannot be changed on update).
    pub practice: Option<i64>,
    /// The user associated with the order.
    pub prescribing_user: Option<i64>,
    /// A list of cardiac tests associated with the order.
    pub tests: Option<Vec<CardiacOrderTest>>,
    /// The date on which the tests should be performed, or were performed (optional).
    #[serde(with = "one_true_date::option")]
    pub test_date: Option<Date>,
}

/// Represents query parameters for searching cardiac orders.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CardiacOrderQueryParams {
    /// The patient the cardiac order belongs to (optional).
    pub patient: Option<i64>,
    /// The practice the cardiac order belongs to (optional).
    pub practice: Option<i64>,
}
