use crate::resource::Resource;
use derive_more::derive::Display;
use serde::{Deserialize, Serialize};

/// Represents the type of Ancillary Company.
#[derive(Clone, Debug, Deserialize, Serialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum AncillaryCompanyType {
    Cardiac,
    Imaging,
    Pulmonary,
    Sleep,
}

/// Represents an Ancillary Company.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AncillaryCompany {
    /// The unique identifier of the ancillary company.
    pub id: i64,
    /// The name of the ancillary company (up to 200 characters).
    pub name: String,
}

impl Resource for AncillaryCompany {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/ancillary_companies"
    }
}

/// Represents query parameters for searching ancillary companies.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AncillaryCompanyQueryParams {
    /// The name of the ancillary company (optional).
    pub name: Option<String>,
    /// The type of ancillary company (required).
    #[serde(rename = "type")]
    pub company_type: Option<AncillaryCompanyType>,
}
