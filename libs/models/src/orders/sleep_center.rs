use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use utils::time::Rfc3339;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SleepCenter {
    /// The unique identifier of the sleep center.
    pub id: i64,
    /// The first line of the address (up to 200 characters).
    pub address_line1: String,
    /// The second line of the address (nullable, up to 40 characters).
    pub address_line2: Option<String>,
    /// The city of the sleep center (up to 50 characters).
    pub city: String,
    /// The name of the company (maps to the Ancillary Company object name attribute, up to 200 characters).
    pub company_name: String,
    /// The ID of the company (maps to the Ancillary Company object ID attribute).
    pub company: i64,
    /// The date when the sleep center was created (ISO 8601 format).
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the sleep center was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The fax number of the sleep center.
    pub fax: String,
    /// The location name of the sleep center (up to 100 characters).
    pub location_name: String,
    /// The phone number of the sleep center.
    pub phone: String,
    /// The practice ID associated with the sleep center (nullable, required for creating).
    pub practice: Option<i64>,
    /// The state of the sleep center (up to 2 characters).
    pub state: String,
    /// The ZIP code of the sleep center (up to 10 characters).
    pub zip: String,
}

impl Resource for SleepCenter {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/sleep_centers"
    }
}

/// Represents query parameters for searching sleep centers.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SleepCenterQueryParams {
    /// The ancillary company associated with the sleep center (optional).
    pub company: Option<i64>,
    /// The location name of the sleep center you're searching for (optional).
    pub location_name: Option<String>,
    /// The practice the sleep center is associated with (optional).
    pub practice: Option<i64>,
}
