use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};
use utils::time::Iso8601;

time::serde::format_description!(one_true_date, Date, "[year]-[month]-[day]");

/// Represents a physical address with optional components.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Address {
    /// The first line of the address.
    pub address_line1: String,
    /// The second line of the address (optional).
    pub address_line2: Option<String>,
    /// The city of the address (optional).
    pub city: Option<String>,
    /// The state of the address (optional).
    pub state: Option<String>,
    /// The ZIP code of the address (optional).
    pub zip: Option<String>,
    /// A list of phone numbers associated with the address.
    pub phones: Vec<Phone>,
}

/// Represents a phone number with a specified type.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Phone {
    /// The phone number.
    pub phone: String,
    /// The type of the phone (e.g., "mobile", "home", "work").
    pub phone_type: String,
}

/// Represents an email address.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Email {
    /// The email address.
    pub email: String,
}

/// Represents insurance information for a patient.
///
/// **Note:** The API requires the `rank` field, but it's not documented.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Insurance {
    /// The member ID of the insurance policy.
    pub member_id: String,
    /// The rank of the insurance (required by the API but undocumented).
    pub rank: String,
}

/// Represents a patient with detailed personal and medical information.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Patient {
    /// The unique identifier of the patient.
    pub id: i64,

    /// The first name of the patient.
    pub first_name: String,
    /// The middle name of the patient (optional).
    pub middle_name: Option<String>,
    /// The last name of the patient.
    pub last_name: String,
    /// The actual name of the patient (optional).
    pub actual_name: Option<String>,
    /// The gender identity of the patient (optional).
    pub gender_identity: Option<GenderIdentity>,
    /// The legal gender marker of the patient (optional).
    pub legal_gender_marker: Option<LegalGenderMarker>,
    /// The pronouns used by the patient (optional).
    pub pronouns: Option<Pronouns>,
    /// The biological sex of the patient.
    pub sex: Sex,
    /// The sexual orientation of the patient (optional).
    pub sexual_orientation: Option<SexualOrientation>,

    /// The ID of the primary physician associated with the patient.
    pub primary_physician: i64,
    /// The ID of the caregiver practice associated with the patient.
    pub caregiver_practice: i64,

    /// The date of birth of the patient.
    #[serde(with = "one_true_date")]
    pub dob: Date,
    /// The Social Security Number of the patient (optional).
    pub ssn: Option<String>,
    /// The race of the patient (optional).
    pub race: Option<Race>,
    /// The ethnicity of the patient (optional).
    pub ethnicity: Option<Ethnicity>,
    /// The preferred language of the patient (optional).
    pub preferred_language: Option<String>,
    /// Additional notes about the patient (optional).
    pub notes: Option<String>,
    /// Indicates whether the patient is marked as VIP.
    pub vip: bool,
    /// A list of tags associated with the patient.
    pub tags: Vec<String>,
    /// The SMS opt-in status of the patient (optional).
    pub sms_opt_in_status: Option<bool>,
    /// The address of the patient (optional).
    pub address: Option<Address>,
    /// A list of phone numbers for the patient (optional).
    pub phones: Option<Vec<Phone>>,
    /// A list of email addresses for the patient (optional).
    pub emails: Option<Vec<Email>>,
    /// The guarantor information for the patient (optional).
    pub guarantor: Option<Guarantor>,
    /// A list of insurance policies for the patient (optional).
    pub insurances: Option<Vec<Insurance>>,
    /// A list of deleted insurance policies for the patient (optional).
    pub deleted_insurances: Option<Vec<Insurance>>,
    /// The patient's preferences (optional).
    pub preference: Option<Preference>,
    /// The emergency contact information for the patient (optional).
    pub emergency_contact: Option<EmergencyContact>,
    /// The previous name(s) of the patient (optional).
    pub previous_name: Option<PreviousName>,
    /// The master patient ID associated with the patient (optional).
    pub master_patient: Option<i64>,
    /// The employer information of the patient (optional).
    pub employer: Option<Employer>,
    /// A list of consents given by the patient (optional).
    pub consents: Option<Vec<Consent>>,
    /// Additional metadata for the patient (optional).
    pub metadata: Option<serde_json::Value>,
    /// The ID of the chart into which the patient was merged (optional).
    pub merged_into_chart: Option<i64>,

    /// The primary care provider's ID for the patient (optional).
    pub primary_care_provider: Option<i64>,
    /// The National Provider Identifier (NPI) of the primary care provider (optional).
    pub primary_care_provider_npi: Option<String>,

    /// The status information of the patient.
    pub patient_status: PatientStatus,

    /// The date when the patient record was created (optional).
    #[serde_as(as = "Option<Iso8601>")]
    pub created_date: Option<OffsetDateTime>,
    /// The date when the patient record was deleted (optional).
    #[serde_as(as = "Option<Iso8601>")]
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents a consent given by the patient.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Consent {
    /// Indicates whether the patient has consented.
    pub consented: bool,
    /// The last date when the consent was modified (optional).
    #[serde_as(as = "Option<Iso8601>")]
    pub last_modified_date: Option<OffsetDateTime>,
    /// The application associated with the consent (optional).
    pub application: Option<String>,
}

/// Represents the employer information of the patient.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Employer {
    /// The employer code (optional).
    pub code: Option<String>,
    /// The name of the employer (optional).
    pub name: Option<String>,
    /// A description of the employer (optional).
    pub description: Option<String>,
}

/// Represents the previous name(s) of the patient.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreviousName {
    /// The previous first name of the patient (optional).
    pub first_name: Option<String>,
    /// The previous middle name of the patient (optional).
    pub middle_name: Option<String>,
    /// The previous last name of the patient (optional).
    pub last_name: Option<String>,
    /// The suffix of the previous name (optional).
    pub suffix: Option<String>,
}

/// Represents the relationship types for an emergency contact.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EmergencyContactRelationship {
    /// Caregiver relationship.
    Caregiver,
    /// Child relationship.
    Child,
    /// Friend relationship.
    Friend,
    /// Grandparent relationship.
    Grandparent,
    /// Guardian relationship.
    Guardian,
    /// Parent relationship.
    Parent,
    /// Sibling relationship.
    Sibling,
    /// Spouse relationship.
    Spouse,
    /// Other relationship.
    Other,
}

/// Represents an emergency contact for the patient.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmergencyContact {
    /// The first name of the emergency contact (optional).
    pub first_name: Option<String>,
    /// The last name of the emergency contact (optional).
    pub last_name: Option<String>,
    /// The relationship to the patient (optional).
    pub relationship: Option<EmergencyContactRelationship>,
    /// The phone number of the emergency contact (optional).
    pub phone: Option<String>,
    /// The first line of the address (optional).
    pub address_line1: Option<String>,
    /// The second line of the address (optional).
    pub address_line2: Option<String>,
    /// The city of the address (optional).
    pub city: Option<String>,
    /// The state of the address (optional).
    pub state: Option<String>,
    /// The ZIP code of the address (optional).
    pub zip: Option<String>,
}

/// Represents the relationship types for a guarantor.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GuarantorRelationship {
    /// Spouse relationship.
    Spouse,
    /// Child relationship.
    Child,
    /// Other relationship.
    Other,
}

/// Represents the patient's preferences, such as preferred pharmacies.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preference {
    /// The NCPDP ID of the first preferred pharmacy (optional).
    pub preferred_pharmacy_1: Option<String>,
    /// The NCPDP ID of the second preferred pharmacy (optional).
    pub preferred_pharmacy_2: Option<String>,
}

/// Represents the guarantor information for the patient.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guarantor {
    /// The unique identifier of the guarantor (optional).
    pub id: Option<i64>,
    /// The address of the guarantor (optional).
    pub address: Option<String>,
    /// The city of the guarantor's address (optional).
    pub city: Option<String>,
    /// The state of the guarantor's address (optional).
    pub state: Option<String>,
    /// The ZIP code of the guarantor's address (optional).
    pub zip: Option<String>,
    /// The phone number of the guarantor (optional).
    pub phone: Option<String>,
    /// The email address of the guarantor (optional).
    pub email: Option<String>,
    /// The relationship of the guarantor to the patient (optional).
    pub relationship: Option<GuarantorRelationship>,
    /// The first name of the guarantor (optional).
    pub first_name: Option<String>,
    /// The last name of the guarantor (optional).
    pub last_name: Option<String>,
    /// The middle name of the guarantor (optional).
    pub middle_name: Option<String>,
}

/// Represents the patient's status, including activity and inactivity reasons.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientStatus {
    /// The date of death if the patient is deceased (optional).
    pub deceased_date: Option<String>,
    /// The reason for the patient's inactivity (optional).
    pub inactive_reason: Option<InactiveReason>,
    /// The date when the patient's status was last changed (optional).
    pub last_status_change: Option<String>,
    /// Additional notes regarding the patient's status (optional).
    pub notes: Option<String>,
    /// The current status of the patient.
    pub status: PatientStatusEnum,
}

/// Represents reasons why a patient might be inactive.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InactiveReason {
    /// Other reasons not specified.
    Other,
    /// The patient left on bad terms.
    PatientLeftOnBadTerms,
    /// The patient left on good terms.
    PatientLeftOnGoodTerms,
    /// The practice ended the relationship.
    PracticeEndedRelationship,
    /// The reason is unknown.
    Unknown,
}

/// Represents the possible statuses of a patient.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PatientStatusEnum {
    /// The patient is active.
    Active,
    /// The patient is deceased.
    Deceased,
    /// The patient is inactive.
    Inactive,
    /// The patient is a prospect.
    Prospect,
}

/// Represents the data required to create a new patient.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientForCreate {
    /// The first name of the patient.
    pub first_name: String,
    /// The last name of the patient.
    pub last_name: String,
    /// The date of birth of the patient.
    #[serde(with = "one_true_date")]
    pub dob: Date,
    /// The biological sex of the patient.
    pub sex: Sex,
    /// The ID of the primary physician.
    pub primary_physician: i64,
    /// The ID of the caregiver practice.
    pub caregiver_practice: i64,
    /// The address of the patient (optional).
    pub address: Option<Address>,
    /// A list of email addresses for the patient (optional).
    pub emails: Option<Vec<Email>>,
    /// A list of insurance policies for the patient.
    pub insurances: Vec<Insurance>,
}

/// Represents query parameters for searching patients.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PatientQueryParams {
    /// The first name of the patient (optional).
    pub first_name: Option<String>,
    /// The last name of the patient (optional).
    pub last_name: Option<String>,
    /// The date of birth of the patient (optional).
    pub dob: Option<String>,
    /// The biological sex of the patient (optional).
    pub sex: Option<String>,
    /// The insurance company associated with the patient (optional).
    pub insurance_company: Option<String>,
    /// The insurance plan associated with the patient (optional).
    pub insurance_plan: Option<String>,
    /// The group ID associated with the patient (optional).
    pub group_id: Option<i64>,
    /// The member ID associated with the patient (optional).
    pub member_id: Option<i64>,
    /// The master patient ID (optional).
    pub master_patient: Option<i64>,
    /// The practice ID associated with the patient (optional).
    pub practice: Option<i64>,
    /// Filter for last modified date greater than (optional).
    pub last_modified__gt: Option<String>,
    /// Filter for last modified date greater than or equal to (optional).
    pub last_modified__gte: Option<String>,
    /// Filter for last modified date less than (optional).
    pub last_modified__lt: Option<String>,
    /// Filter for last modified date less than or equal to (optional).
    pub last_modified__lte: Option<String>,
    /// The maximum number of results to return (optional).
    pub limit: Option<i32>,
    /// The offset for pagination (optional).
    pub offset: Option<i32>,
}

// The implementation of Patient methods is omitted for brevity.
// impl Patient {
//     pub async fn get(elation_patient_id: i64) -> Result<Patient> {
//         ...
//     }
//
//     pub async fn create(patient_fc: &PatientForCreate) -> Result<Patient> {
//         ...
//     }
//
//     pub async fn delete(id: i64) -> Result<String> {
//         ...
//     }
//
//     pub async fn find(params: PatientQueryParams) -> Result<client::PaginatedResponse<Patient>> {
//         ...
//     }
// }

/// Represents the race of a patient.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Race {
    /// No race specified.
    #[serde(rename = "No race specified")]
    NoRaceSpecified,
    /// American Indian or Alaska Native.
    #[serde(rename = "American Indian or Alaska Native")]
    AmericanIndianOrAlaskaNative,
    /// Asian race.
    #[serde(rename = "Asian")]
    Asian,
    /// Black or African American.
    #[serde(rename = "Black or African American")]
    BlackOrAfricanAmerican,
    /// Native Hawaiian or Other Pacific Islander.
    #[serde(rename = "Native Hawaiian or Other Pacific Islander")]
    NativeHawaiianOrOtherPacificIslander,
    /// White race.
    #[serde(rename = "White")]
    White,
    /// Declined to specify race.
    #[serde(rename = "Declined to specify")]
    DeclinedToSpecify,
}

/// Represents the ethnicity of a patient.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Ethnicity {
    /// No ethnicity specified.
    #[serde(rename = "No ethnicity specified")]
    NoEthnicitySpecified,
    /// Hispanic or Latino ethnicity.
    #[serde(rename = "Hispanic or Latino")]
    HispanicOrLatino,
    /// Not Hispanic or Latino.
    #[serde(rename = "Not Hispanic or Latino")]
    NotHispanicOrLatino,
    /// Declined to specify ethnicity.
    #[serde(rename = "Declined to specify")]
    DeclinedToSpecify,
}

/// Represents the gender identity of a patient.
#[derive(Clone, Debug, derive_more::Display, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GenderIdentity {
    /// Unknown gender identity.
    Unknown,
    /// Man.
    Man,
    /// Woman.
    Woman,
    /// Transgender man.
    TransgenderMan,
    /// Transgender woman.
    TransgenderWoman,
    /// Nonbinary gender identity.
    Nonbinary,
    /// Option not listed.
    OptionNotListed,
    /// Prefer not to say.
    PreferNotToSay,
    /// Two-spirit gender identity.
    TwoSpirit,
}

/// Represents the legal gender marker of a patient.
#[derive(Clone, Debug, derive_more::Display, Deserialize, Serialize)]
pub enum LegalGenderMarker {
    /// Male.
    M,
    /// Female.
    F,
    /// Nonbinary or unspecified.
    X,
    /// Unknown.
    U,
}

/// Represents the pronouns used by a patient.
#[derive(Clone, Debug, derive_more::Display, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Pronouns {
    /// He/Him/His pronouns.
    HeHimHis,
    /// She/Her/Hers pronouns.
    SheHerHers,
    /// They/Them/Theirs pronouns.
    TheyThemTheirs,
    /// Pronouns not listed.
    NotListed,
}

/// Represents the biological sex of a patient.
#[derive(Clone, Debug, derive_more::Display, Deserialize, Serialize, Eq, PartialEq)]
pub enum Sex {
    /// Male.
    Male,
    /// Female.
    Female,
    /// Other.
    Other,
    /// Unknown.
    Unknown,
}

/// Represents the sexual orientation of a patient.
#[derive(Clone, Debug, derive_more::Display, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SexualOrientation {
    /// Unknown sexual orientation.
    Unknown,
    /// Straight.
    Straight,
    /// Gay.
    Gay,
    /// Bisexual.
    Bisexual,
    /// Option not listed.
    OptionNotListed,
    /// Prefer not to say.
    PreferNotToSay,
    /// Lesbian.
    Lesbian,
    /// Queer.
    Queer,
    /// Asexual.
    Asexual,
}
