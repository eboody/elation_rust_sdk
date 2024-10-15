use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::resource::Resource;

/// Represents an insurance card for a patient, including both primary and secondary insurance.
///
/// Insurance card images are returned as AWS S3 presigned URLs that grant time-limited permission
/// to download the images.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsuranceCard {
    /// The rank of the insurance card (1 for primary, 2 for secondary).
    pub rank: i32,

    /// The images of the insurance card (front and back).
    pub images: Vec<InsuranceCardImage>,
}

/// Represents an image of the insurance card, either front or back.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsuranceCardImage {
    /// The side of the insurance card (1 for front, 2 for back).
    pub side: i32,

    /// The URL to access the image (AWS S3 presigned URL).
    pub url: String,

    /// The time-to-live (TTL) in seconds for the presigned URL.
    pub ttl: i32,
}

/// Represents the data required to create a new insurance card.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsuranceCardForCreate {
    /// The rank of the insurance card (1 for primary, 2 for secondary).
    pub rank: i32,

    /// The images of the insurance card (front and back).
    pub images: Vec<InsuranceCardImageForCreate>,
}

/// Represents the data required to create a new insurance card image.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsuranceCardImageForCreate {
    /// The side of the insurance card (1 for front, 2 for back).
    pub side: i32,

    /// The URL to access the image (AWS S3 presigned URL).
    pub url: String,

    /// The time-to-live (TTL) in seconds for the presigned URL.
    pub ttl: i32,
}

/// Represents the data required to update an existing insurance card.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InsuranceCardForUpdate {
    /// The rank of the insurance card (optional).
    pub rank: Option<i32>,

    /// The images of the insurance card (front and back) (optional).
    pub images: Option<Vec<InsuranceCardImageForUpdate>>,
}

/// Represents the data required to update an existing insurance card image.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InsuranceCardImageForUpdate {
    /// The side of the insurance card (optional).
    pub side: Option<i32>,

    /// The URL to access the image (optional).
    pub url: Option<String>,

    /// The time-to-live (TTL) in seconds for the presigned URL (optional).
    pub ttl: Option<i32>,
}

impl Resource for InsuranceCard {
    type Id = i32;

    fn endpoint() -> &'static str {
        "/insurance_cards"
    }
}
