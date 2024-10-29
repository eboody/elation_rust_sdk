use super::lab_order_compendium::LabOrderCompendium;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabVendor {
    /// The unique identifier of the lab vendor.
    pub id: i64,

    /// The ID of the practice that created the lab vendor.
    /// Can be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_created: Option<i64>,

    /// The name of the lab vendor.
    pub name: String,

    /// The display name of the lab vendor.
    pub display_name: String,

    /// Indicates if the lab vendor has an order compendium.
    /// This is the same field as `has_test_compendium`.
    pub has_order_compendium: bool,

    /// Indicates if the lab vendor has a test compendium.
    pub has_test_compendium: bool,

    /// Indicates if results integration is available.
    pub results_integration_available: bool,

    /// Indicates if orders integration is available.
    pub orders_integration_available: bool,

    /// A list of compendiums associated with the lab vendor.
    #[serde(default)]
    pub compendiums: Vec<LabOrderCompendium>,

    /// The default compendium for the lab vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_compendium: Option<LabOrderCompendium>,
}

impl Resource for LabVendor {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/lab_vendors"
    }
}

/// Represents the data required to create a new lab vendor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabVendorForCreate {
    /// The name of the lab vendor.
    /// Value must be 'Lab' for laboratory reports.
    pub name: String,

    /// The display name of the lab vendor.
    pub display_name: String,

    /// The ID of the practice that created the lab vendor.
    pub practice_created: i64,

    /// Indicates if the lab vendor has a test compendium.
    pub has_test_compendium: bool,

    /// Indicates if results integration is available.
    pub results_integration_available: bool,

    /// Indicates if orders integration is available.
    pub orders_integration_available: bool,
}

/// Represents the data required to update an existing lab vendor.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LabVendorForUpdate {
    /// The name of the lab vendor.
    /// Value must be 'Lab' for laboratory reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The display name of the lab vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The ID of the practice that created the lab vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_created: Option<i64>,

    /// Indicates if the lab vendor has a test compendium.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_test_compendium: Option<bool>,

    /// Indicates if results integration is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_integration_available: Option<bool>,

    /// Indicates if orders integration is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders_integration_available: Option<bool>,
}

/// Represents query parameters for searching lab vendors.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LabVendorQueryParams {
    /// Names of the lab vendors to find.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,

    /// The IDs of practices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_created: Option<Vec<i64>>,
}
