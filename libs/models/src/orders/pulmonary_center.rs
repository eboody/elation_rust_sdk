use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use utils::time::Rfc3339;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PulmonaryCenter {
    /// The unique identifier of the pulmonary center.
    pub id: i64,
    /// The first line of the address.
    pub address_line1: String,
    /// The second line of the address (nullable).
    pub address_line2: Option<String>,
    /// The city of the pulmonary center.
    pub city: String,
    /// The name of the company (maps to the Ancillary Company object name attribute).
    pub company_name: String,
    /// The ID of the company (maps to the Ancillary Company object ID attribute).
    pub company: i64,
    /// The date when the pulmonary center was created (ISO 8601 format).
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the pulmonary center was deleted (nullable).
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    /// The fax number of the pulmonary center.
    pub fax: String,
    /// The location name of the pulmonary center.
    pub location_name: String,
    /// The phone number of the pulmonary center.
    pub phone: String,
    /// The practice ID associated with the pulmonary center (nullable).
    pub practice: Option<i64>,
    /// The state of the pulmonary center.
    pub state: String,
    /// The ZIP code of the pulmonary center.
    pub zip: String,
}

impl Resource for PulmonaryCenter {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/pulmonary_centers"
    }
}

/// Represents query parameters for searching pulmonary centers.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PulmonaryCenterQueryParams {
    /// The ancillary company associated with the pulmonary center (optional).
    pub company: Option<i64>,
    /// The location name of the pulmonary center (optional).
    pub location_name: Option<String>,
    /// The practice ID associated with the pulmonary center (optional).
    pub practice: Option<i64>,
}
