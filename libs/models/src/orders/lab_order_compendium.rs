use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::resource::Resource;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderCompendium {
    /// The unique identifier of the lab order compendium.
    pub id: i64,

    /// The lab vendor ID.
    pub lab_vendor: i64,

    /// The name of the compendium.
    pub name: String,

    /// The last updated date and time.
    pub last_updated: OffsetDateTime,

    /// The code of the compendium.
    pub code: String,

    /// The date when the compendium was created.
    pub created_date: OffsetDateTime,

    /// The date when the compendium was deleted (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<OffsetDateTime>,
}

impl Resource for LabOrderCompendium {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/lab_order_compendiums"
    }
}

/// Represents the data required to create a new lab order compendium.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderCompendiumForCreate {
    /// The lab vendor ID.
    pub lab_vendor: i64,

    /// Must be unique with each individual 'lab_vendor'.
    pub code: String,

    /// Must be unique with each individual 'lab_vendor'.
    pub name: String,
}

/// Represents the data required to update an existing lab order compendium.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LabOrderCompendiumForUpdate {
    /// The lab vendor ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lab_vendor: Option<i64>,

    /// Must be unique with each individual 'lab_vendor'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Must be unique with each individual 'lab_vendor'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Represents query parameters for searching lab order compendiums.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LabOrderCompendiumQueryParams {
    /// Names of the compendiums to find.
    pub name: Option<Vec<String>>,

    /// Codes of the compendiums to find.
    pub code: Option<Vec<String>>,

    /// The IDs of lab vendors.
    pub lab_vendor: Option<Vec<i64>>,

    /// The IDs of practices.
    ///
    /// Note: This field corresponds to `lab_vendor__practice_created` in the API.
    pub practice_created: Option<Vec<i64>>,
}
