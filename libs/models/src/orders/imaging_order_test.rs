use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use utils::time::Rfc3339;

use crate::resource::Resource;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImagingOrderTest {
    /// The code of the imaging order test (nullable, up to 50 characters).
    pub code: Option<String>,
    /// The date when the imaging order test was created.
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the imaging order test was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The unique identifier of the imaging order test.
    pub id: i64,
    /// The name of the imaging order test (required for creating, up to 255 characters).
    pub name: String,
    /// The practice ID associated with the imaging order test (nullable, required for creating).
    pub practice: Option<i64>,
}

impl Resource for ImagingOrderTest {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/imaging_order_tests"
    }
}

/// Represents the data required to create a new imaging order test.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImagingOrderTestForCreate {
    /// The code of the imaging order test (optional, up to 50 characters).
    pub code: Option<String>,
    /// The name of the imaging order test (required, up to 255 characters).
    pub name: String,
    /// The practice ID associated with the imaging order test (required).
    pub practice: i64,
}

/// Represents query parameters for searching imaging order tests.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ImagingOrderTestQueryParams {
    /// The name of the imaging order test you're searching for (optional).
    pub name: Option<String>,
    /// The practice the imaging order test is associated with (optional).
    pub practice: Option<i64>,
}
