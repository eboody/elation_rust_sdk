use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use utils::time::Rfc3339;

use crate::resource::Resource;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardiacCenter {
    /// The unique identifier of the cardiac center.
    pub id: i64,
    /// The first line of the address (up to 200 characters).
    pub address_line1: String,
    /// The second line of the address (nullable, up to 40 characters).
    pub address_line2: Option<String>,
    /// The city of the cardiac center (up to 50 characters).
    pub city: String,
    /// The company name (maps to the Ancillary Company object name attribute).
    pub company_name: String,
    /// Maps to the Ancillary Company object ID attribute.
    pub company: i64,
    /// The date when the cardiac center was created (ISO 8601 format).
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the cardiac center was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The fax number of the cardiac center.
    pub fax: Option<String>,
    /// The location name of the cardiac center (required for creating, up to 100 characters).
    pub location_name: String,
    /// The phone number of the cardiac center.
    pub phone: Option<String>,
    /// The practice ID associated with the cardiac center (nullable, required for creating).
    pub practice: Option<i64>,
    /// The state where the cardiac center is located (up to 2 characters).
    pub state: String,
    /// The ZIP code of the cardiac center (up to 10 characters).
    pub zip: String,
}

impl Resource for CardiacCenter {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/cardiac_centers"
    }
}

/// Represents the data required to create a new cardiac center.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardiacCenterForCreate {
    /// The first line of the address (up to 200 characters).
    pub address_line1: String,
    /// The second line of the address (nullable, up to 40 characters).
    pub address_line2: Option<String>,
    /// The city of the cardiac center (up to 50 characters).
    pub city: String,
    /// The company name (maps to the Ancillary Company object name attribute).
    pub company_name: String,
    /// Maps to the Ancillary Company object ID attribute.
    pub company: i64,
    /// The fax number of the cardiac center.
    pub fax: Option<String>,
    /// The location name of the cardiac center (required, up to 100 characters).
    pub location_name: String,
    /// The phone number of the cardiac center.
    pub phone: Option<String>,
    /// The practice ID associated with the cardiac center (required).
    pub practice: i64,
    /// The state where the cardiac center is located (up to 2 characters).
    pub state: String,
    /// The ZIP code of the cardiac center (up to 10 characters).
    pub zip: String,
}

/// Represents query parameters for searching cardiac centers.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CardiacCenterQueryParams {
    /// The ancillary company associated with the cardiac center (optional).
    pub company: Option<i64>,
    /// The location name of the cardiac center you're searching for (optional).
    pub location_name: Option<String>,
    /// The practice the cardiac center is associated with (optional).
    pub practice: Option<i64>,
}
