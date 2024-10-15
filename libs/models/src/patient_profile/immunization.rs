use super::{Vaccine, VaccineForCreate, VaccineForUpdate};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};

use crate::resource::Resource;

/// Represents an immunization object in the patient profile.
///
/// This object contains information about the vaccine, administering and ordering physicians,
/// site, method, and other important details like manufacturer, lot number, and expiration date.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Immunization {
    /// The ID of the immunization.
    pub id: i64,

    /// The date the immunization was administered.
    pub administered_date: OffsetDateTime,

    /// The ID of the administering physician.
    pub administering_physician: i64,

    /// The ID of the ordering physician.
    pub ordering_physician: i64,

    /// The description of the immunization.
    pub description: String,

    /// The reason for the immunization (optional).
    pub reason: Option<String>,

    /// Vaccine information associated with the immunization.
    pub vaccine: Vaccine,

    /// The quantity of the dose administered (optional).
    pub qty: Option<f64>,

    /// The units of the quantity administered (optional).
    pub qty_units: Option<String>,

    /// The lot number of the vaccine (optional).
    pub lot_number: Option<String>,

    /// The name of the manufacturer (optional).
    pub manufacturer_name: Option<String>,

    /// The manufacturer code (optional).
    pub manufacturer_code: Option<String>,

    /// The expiration date of the vaccine (optional).
    pub expiration_date: Option<Date>,

    /// The VIS (Vaccine Information Statement) provided to the patient (optional).
    pub vis: Option<String>,

    /// The method of administration (e.g., "Intramuscular").
    pub method: String,

    /// The site where the vaccine was administered (e.g., "R Deltoid (RD)").
    pub site: String,

    /// Any notes about the immunization (optional).
    pub notes: Option<String>,

    /// The publicity code associated with reminders or recalls (optional).
    pub publicity_code: Option<String>,

    /// The VFC (Vaccines for Children) eligibility (optional).
    pub vfc_eligibility: Option<String>,

    /// The funding source for the immunization (optional).
    pub funding_source: Option<String>,

    /// The source of the immunization information (optional).
    pub info_source: Option<String>,

    /// Indicates whether sharing of this information is allowed (optional).
    pub allowed_sharing: Option<bool>,

    /// The ID of the practice associated with the immunization.
    pub practice: i64,

    /// The ID of the patient associated with the immunization.
    pub patient: i64,

    /// The date the immunization was created (read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub created_date: Option<OffsetDateTime>,

    /// The date the immunization was deleted (optional, read-only).
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac>")]
    pub deleted_date: Option<OffsetDateTime>,

    /// The dose in the series (optional).
    pub dose_in_series: Option<String>,

    /// The priority population for the immunization (optional).
    pub priority_population: Option<String>,

    /// The National Drug Code (NDC) (optional).
    pub ndc: Option<String>,

    /// Whether the patient provided consent for the immunization (optional).
    pub patient_consent: Option<bool>,
}

///// Represents vaccine information in an immunization record.
//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Vaccine {
//    /// The name of the vaccine.
//    pub name: Option<String>,
//
//    /// The CVX code of the vaccine.
//    pub cvx: i64,
//
//    /// The CDC name of the vaccine.
//    pub cdc_name: Option<String>,
//
//    /// The CDC type of the vaccine.
//    pub cdc_type: Option<String>,
//}

/// Represents the data required to create a new immunization.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImmunizationForCreate {
    /// The date the immunization was administered.
    pub administered_date: OffsetDateTime,

    /// The ID of the administering physician.
    pub administering_physician: i64,

    /// The ID of the ordering physician.
    pub ordering_physician: i64,

    /// The description of the immunization.
    pub description: String,

    /// Vaccine information associated with the immunization.
    pub vaccine: VaccineForCreate,

    /// The method of administration (e.g., "Intramuscular").
    pub method: String,

    /// The site where the vaccine was administered (e.g., "R Deltoid (RD)").
    pub site: String,

    /// The ID of the patient associated with the immunization.
    pub patient: i64,
}

/// Represents the data required to update an existing immunization.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ImmunizationForUpdate {
    /// The date the immunization was administered (optional).
    pub administered_date: Option<OffsetDateTime>,

    /// The ID of the administering physician (optional).
    pub administering_physician: Option<i64>,

    /// The ID of the ordering physician (optional).
    pub ordering_physician: Option<i64>,

    /// The description of the immunization (optional).
    pub description: Option<String>,

    /// Vaccine information associated with the immunization (optional).
    pub vaccine: Option<VaccineForUpdate>,

    /// The method of administration (optional).
    pub method: Option<String>,

    /// The site where the vaccine was administered (optional).
    pub site: Option<String>,
}

impl Resource for Immunization {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/immunizations"
    }
}
