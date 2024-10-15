use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::resource::Resource;

/// Represents a patient's photo, which includes metadata such as file type, size, dimensions, and more.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientPhoto {
    /// The ID of the patient photo.
    pub id: i64,

    /// The ID of the patient associated with the photo.
    pub patient: i64,

    /// The ID of the practice associated with the photo.
    pub practice: i64,

    /// The URL to access the photo.
    pub file: String,

    /// The content type of the photo (e.g., image/jpeg).
    pub content_type: String,

    /// The original filename of the photo.
    pub original_filename: String,

    /// The size of the photo file in bytes.
    pub file_size: i64,

    /// The width of the photo.
    pub width: i32,

    /// The height of the photo.
    pub height: i32,

    /// The last time the photo was updated.
    pub last_updated: OffsetDateTime,

    /// The date the photo was created.
    pub created_date: OffsetDateTime,
}

/// Represents the data required to create a new patient photo.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientPhotoForCreate {
    /// The ID of the patient associated with the photo.
    pub patient: i64,

    /// The ID of the practice associated with the photo.
    pub practice: i64,

    /// The content type of the photo (e.g., image/jpeg).
    pub content_type: String,

    /// The original filename of the photo.
    pub original_filename: String,

    /// The size of the photo file in bytes.
    pub file_size: i64,

    /// The width of the photo.
    pub width: i32,

    /// The height of the photo.
    pub height: i32,
}

/// Represents the data required to update an existing patient photo.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PatientPhotoForUpdate {
    /// The content type of the photo (e.g., image/jpeg) (optional).
    pub content_type: Option<String>,

    /// The original filename of the photo (optional).
    pub original_filename: Option<String>,

    /// The size of the photo file in bytes (optional).
    pub file_size: Option<i64>,

    /// The width of the photo (optional).
    pub width: Option<i32>,

    /// The height of the photo (optional).
    pub height: Option<i32>,
}

impl Resource for PatientPhoto {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/patient_photos"
    }
}
