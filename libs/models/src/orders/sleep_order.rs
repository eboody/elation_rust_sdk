use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};
use utils::time::Rfc3339;

use super::{sleep_order_test::SleepOrderTest, Resolution};
use crate::{resource::Resource, Icd10Code};

time::serde::format_description!(one_true_date, Date, "[year]-[month]-[day]");

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SleepOrder {
    /// The unique identifier of the sleep order.
    pub id: i64,
    /// The ID of the ancillary company the order is associated with.
    pub ancillary_company: i64,
    /// Array of Physician IDs to copy for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccs: Option<Vec<i64>>,
    /// The chart date of the order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// The clinical reason for the order.
    pub clinical_reason: String,
    /// Whether the order is confidential.
    pub confidential: bool,
    /// The date when the sleep order was created (ISO 8601 format).
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the sleep order was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The document date of the order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// How to notify the patient of the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd10_codes: Option<Vec<Icd10Code>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_by: Option<i64>,
    /// The resolution state of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    /// The sleep center ID associated with the order (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_center: Option<i64>,
    /// The date on which the tests should be performed, or were performed.
    #[serde(with = "one_true_date::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_date: Option<Date>,
    /// An array of sleep order tests associated with the order.
    pub tests: Vec<SleepOrderTest>,
}

impl Resource for SleepOrder {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/sleep_orders"
    }
}

/// Represents the data required to create a new sleep order.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SleepOrderForCreate {
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
    pub icd10_codes: Option<Vec<Icd10Code>>,
    /// The patient ID associated with the order.
    pub patient: i64,
    /// The practice ID associated with the order.
    pub practice: i64,
    /// The user ID associated with the order.
    pub prescribing_user: i64,
    /// The sleep center ID associated with the order (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_center: Option<i64>,
    /// The tests associated with the order.
    pub tests: Vec<SleepOrderTestForOrder>,
}

/// Represents the data required to update an existing sleep order.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SleepOrderForUpdate {
    /// The ID of the ancillary company the order is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_company: Option<i64>,
    /// Array of Physician IDs to copy for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccs: Option<Vec<i64>>,
    /// The chart date of the order.
    #[serde(with = "one_true_date::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_date: Option<Date>,
    /// The clinical reason for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_reason: Option<String>,
    /// Whether the order is confidential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    /// The document date of the order.
    #[serde(with = "one_true_date::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_date: Option<Date>,
    /// How to notify the patient of the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_method: Option<String>,
    /// Array of ICD-10 diagnosis codes associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd10_codes: Option<Vec<Icd10Code>>,
    /// The user ID associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescribing_user: Option<i64>,
    /// The sleep center ID associated with the order (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_center: Option<i64>,
    /// The tests associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<Vec<SleepOrderTestForOrder>>,
}

/// Represents query parameters for searching sleep orders.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SleepOrderQueryParams {
    /// The patient the order belongs to (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<i64>,
    /// The practice the order belongs to (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice: Option<i64>,
}

/// Represents a sleep order test associated with an order.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SleepOrderTestForOrder {
    /// The ID of the sleep order test.
    pub id: i64,
}
