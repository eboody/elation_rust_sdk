use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::resource::Resource;

/// Represents an appointment type in the scheduling system.
///
/// Appointment types define various parameters related to appointments such as abbreviation, color,
/// duration, and more. These types can include patient forms, visit note templates, and support telehealth.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppointmentType {
    /// The abbreviation for the appointment type.
    pub abbreviation: Option<String>,

    /// The color associated with the appointment type.
    pub color: Option<String>,

    /// The default duration for this type of appointment (in minutes).
    pub default_duration: i32,

    /// A description of the appointment type.
    pub description: Option<String>,

    /// A list of IDs representing patient forms associated with the appointment type.
    pub patient_forms: Vec<i64>,

    /// The number of hours before the appointment when patient forms are required (optional).
    pub patient_form_hours_prior: Option<i32>,

    /// The ID of the appointment type.
    pub id: i64,

    /// Indicates if the appointment type is for telehealth.
    pub is_telehealth: bool,

    /// The name of the appointment type.
    pub name: String,

    /// The practice ID associated with this appointment type.
    pub practice: i64,

    /// Indicates if the appointment is patient-bookable.
    pub patient_bookable: bool,

    /// The sequence or order in which the appointment type appears.
    pub sequence: i32,

    /// The format of the visit note.
    pub visit_note_format: Option<String>,

    /// A list of IDs representing visit note templates associated with the appointment type.
    pub visit_note_templates: Vec<i64>,

    /// The type of visit note (optional).
    pub visit_note_type: Option<String>,

    /// The date the appointment type was created (read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<OffsetDateTime>,

    /// The date the appointment type was deleted (optional, read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents the data required to create a new appointment type.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppointmentTypeForCreate {
    /// The abbreviation for the appointment type.
    pub abbreviation: Option<String>,

    /// The color associated with the appointment type.
    pub color: Option<String>,

    /// The default duration for this type of appointment (in minutes).
    pub default_duration: i32,

    /// A description of the appointment type (optional).
    pub description: Option<String>,

    /// A list of IDs representing patient forms associated with the appointment type.
    pub patient_forms: Vec<i64>,

    /// The number of hours before the appointment when patient forms are required (optional).
    pub patient_form_hours_prior: Option<i32>,

    /// Indicates if the appointment type is for telehealth.
    pub is_telehealth: bool,

    /// The name of the appointment type.
    pub name: String,

    /// The practice ID associated with this appointment type.
    pub practice: i64,

    /// Indicates if the appointment is patient-bookable.
    pub patient_bookable: bool,

    /// The sequence or order in which the appointment type appears (optional).
    pub sequence: i32,

    /// The format of the visit note (optional).
    pub visit_note_format: Option<String>,

    /// A list of IDs representing visit note templates associated with the appointment type (optional).
    pub visit_note_templates: Vec<i64>,

    /// The type of visit note (optional).
    pub visit_note_type: Option<String>,
}

/// Represents the data required to update an existing appointment type.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AppointmentTypeForUpdate {
    /// The abbreviation for the appointment type (optional).
    pub abbreviation: Option<String>,

    /// The color associated with the appointment type (optional).
    pub color: Option<String>,

    /// The default duration for this type of appointment (optional).
    pub default_duration: Option<i32>,

    /// A description of the appointment type (optional).
    pub description: Option<String>,

    /// A list of IDs representing patient forms associated with the appointment type (optional).
    pub patient_forms: Option<Vec<i64>>,

    /// The number of hours before the appointment when patient forms are required (optional).
    pub patient_form_hours_prior: Option<i32>,

    /// Indicates if the appointment type is for telehealth (optional).
    pub is_telehealth: Option<bool>,

    /// The name of the appointment type (optional).
    pub name: Option<String>,

    /// The practice ID associated with this appointment type (optional).
    pub practice: Option<i64>,

    /// Indicates if the appointment is patient-bookable (optional).
    pub patient_bookable: Option<bool>,

    /// The sequence or order in which the appointment type appears (optional).
    pub sequence: Option<i32>,

    /// The format of the visit note (optional).
    pub visit_note_format: Option<String>,

    /// A list of IDs representing visit note templates associated with the appointment type (optional).
    pub visit_note_templates: Option<Vec<i64>>,

    /// The type of visit note (optional).
    pub visit_note_type: Option<String>,
}

impl Resource for AppointmentType {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/appointment_types"
    }
}
