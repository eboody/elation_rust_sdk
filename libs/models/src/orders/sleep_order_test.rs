use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use utils::time::Rfc3339;

use crate::resource::Resource;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SleepOrderTest {
    /// The unique identifier of the sleep order test.
    pub id: i64,
    /// The code of the sleep order test (nullable, up to 50 characters).
    pub code: Option<String>,
    /// The name of the sleep order test (up to 255 characters).
    pub name: String,
    /// The practice ID associated with the sleep order test (nullable).
    pub practice: Option<i64>,
    /// The date when the sleep order test was created (ISO 8601 format).
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the sleep order test was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
}

impl Resource for SleepOrderTest {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/sleep_order_tests"
    }
}

/// Represents the data required to create a new sleep order test.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SleepOrderTestForCreate {
    /// The code of the sleep order test (nullable, up to 50 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The name of the sleep order test (required, up to 255 characters).
    pub name: String,
    /// The practice ID associated with the sleep order test (required).
    pub practice: i64,
}

/// Represents the data required to update an existing sleep order test.
/// Assuming the API supports updates (not specified, but included for completeness).
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SleepOrderTestForUpdate {
    /// The code of the sleep order test (nullable, up to 50 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The name of the sleep order test (up to 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The practice ID associated with the sleep order test.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice: Option<i64>,
}

/// Represents query parameters for searching sleep order tests.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SleepOrderTestQueryParams {
    /// The name of the sleep order test you're searching for (optional).
    pub name: Option<String>,
    /// The practice the sleep order test is associated with (optional).
    pub practice: Option<i64>,
}
