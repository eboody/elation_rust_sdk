use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use utils::time::Rfc3339;

use crate::resource::Resource;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardiacOrderTest {
    /// The code of the cardiac order test (nullable, up to 50 characters).
    pub code: Option<String>,
    /// The date when the cardiac order test was created.
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the cardiac order test was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The unique identifier of the cardiac order test.
    pub id: i64,
    /// The name of the cardiac order test (required for creating, up to 255 characters).
    pub name: String,
    /// The practice ID associated with the cardiac order test (nullable, required for creating).
    pub practice: Option<i64>,
}

impl Resource for CardiacOrderTest {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/cardiac_order_tests"
    }
}

/// Represents the data required to create a new cardiac order test.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardiacOrderTestForCreate {
    /// The code of the cardiac order test (optional, up to 50 characters).
    pub code: Option<String>,
    /// The name of the cardiac order test (required, up to 255 characters).
    pub name: String,
    /// The practice ID associated with the cardiac order test (required).
    pub practice: i64,
}

/// Represents query parameters for searching cardiac order tests.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CardiacOrderTestQueryParams {
    /// The name of the cardiac order test you're searching for (optional).
    pub name: Option<String>,
    /// The practice the cardiac order test is associated with (optional).
    pub practice: Option<i64>,
}
