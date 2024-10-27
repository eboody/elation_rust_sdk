use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use utils::time::Rfc3339;

use crate::resource::Resource;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImagingCenter {
    /// The unique identifier of the imaging center.
    pub id: i64,
    /// The first line of the address (up to 200 characters).
    pub address_line1: String,
    /// The second line of the address (nullable, up to 40 characters).
    pub address_line2: Option<String>,
    /// The city of the imaging center (up to 50 characters).
    pub city: String,
    /// The company name (maps to the Ancillary Company object name attribute).
    pub company_name: String,
    /// Maps to the Ancillary Company object ID attribute.
    pub company: i64,
    /// The date when the imaging center was created (ISO 8601 format).
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the imaging center was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The fax number of the imaging center.
    pub fax: Option<String>,
    /// The location name of the imaging center (up to 100 characters).
    pub location_name: String,
    /// The phone number of the imaging center.
    pub phone: Option<String>,
    /// The practice ID associated with the imaging center (nullable, required for creating).
    pub practice: Option<i64>,
    /// The state where the imaging center is located (up to 2 characters).
    pub state: String,
    /// The ZIP code of the imaging center (up to 10 characters).
    pub zip: String,
}

impl Resource for ImagingCenter {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/imaging_centers"
    }
}

/// Represents query parameters for searching imaging centers.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ImagingCenterQueryParams {
    /// The ancillary company associated with the imaging center (optional).
    pub company: Option<i64>,
    /// The location name of the imaging center you're searching for (optional).
    pub location_name: Option<String>,
    /// The practice the imaging center is associated with (optional).
    pub practice: Option<i64>,
}
