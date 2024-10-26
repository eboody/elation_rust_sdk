use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use super::Resolution;

/// Represents a Lab Order for a patient.
///
/// A lab order contains details such as the ordering physician, patient, requisition, and resolution.
/// It may also contain details regarding the facility, test dates, and the lab vendor involved.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrder {
    /// The ID of the lab order.
    pub id: i64,

    /// The type of billing for the lab order.
    pub bill_type: Option<String>,

    /// The date the lab order was added to the patient’s chart.
    pub chart_date: OffsetDateTime,

    /// Whether the lab order is marked confidential.
    pub confidential: bool,

    /// The content of the lab order report (nested object).
    pub content: Option<LabOrderContent>,

    /// The date the lab order was created.
    pub created_date: OffsetDateTime,

    /// The date the lab order document was created.
    pub document_date: OffsetDateTime,

    /// The ID of the ordering physician.
    pub ordering_physician: i64,

    /// The ID of the patient.
    pub patient: i64,

    /// The ID of the practice.
    pub practice: i64,

    /// The requisition ID of the lab order.
    pub requisition: i64,

    /// The resolution of the lab order.
    pub resolution: Option<Resolution>,

    /// The ID of the physician who signed off on the lab order.
    pub signed_by: i64,

    /// The date the lab order was signed.
    pub signed_date: OffsetDateTime,

    /// The ID of the vendor providing the lab.
    pub vendor: i64,

    /// Printable view URL for the lab order.
    pub printable_view: Option<String>,
}

/// Represents the content of the lab order.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderContent {
    /// The ID of the lab order content.
    pub id: i64,

    /// The stat method for the lab order (if any).
    pub stat_method: Option<String>,

    /// Notes for the patient.
    pub patient_instructions: Option<String>,

    /// Notes for the lab.
    pub test_center_notes: Option<String>,

    /// The fasting method for the patient (if any).
    pub fasting_method: Option<String>,

    /// The frequency of a standing order (if applicable).
    pub standing_order_frequency: Option<String>,

    /// The date at which a standing order should be canceled (if applicable).
    pub standing_order_end_date: Option<OffsetDateTime>,

    /// The date and time when the specimen was collected (if applicable).
    pub collection_datetime: Option<OffsetDateTime>,

    /// The ICD-10 diagnosis codes associated with the order.
    pub icd10_codes: Vec<String>,

    /// The list of lab order tests.
    pub tests: Vec<LabOrderTest>,
}

/// Represents a lab order test.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderTest {
    /// The ID of the lab order test.
    pub id: i64,

    /// The name of the lab order test.
    pub name: String,

    /// Any code associated with the lab order test.
    pub code: Option<String>,

    /// The procedure class of the test (if applicable).
    pub procedure_class: Option<String>,

    /// The ID of the practice that created the test (if applicable).
    pub practice_created: Option<i64>,

    /// The ID of the lab vendor for the test (if applicable).
    pub lab_vendor: Option<i64>,

    /// The ID of the compendium providing the lab vendor's test (if applicable).
    pub compendium: Option<i64>,

    /// A list of CPT codes associated with the test.
    pub cpts: Vec<String>,

    /// A list of synonyms for the test.
    pub synonyms: Vec<String>,

    /// A list of Ask on Entry (AOE) questions that need to be answered.
    pub questions: Vec<AOEQuestion>,

    /// The date the test was created.
    pub created_date: OffsetDateTime,

    /// The date the test was deleted (if applicable).
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents an Ask on Entry (AOE) question for a lab order test.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AOEQuestion {
    /// The question text.
    pub question: String,

    /// The answer provided for the question.
    pub answer: Option<String>,
}

///// Represents the resolution of the lab order.
//#[serde_as]
//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Resolution {
//    /// The ID of the resolution.
//    pub id: i64,
//
//    /// The document ID of the lab order.
//    pub document: i64,
//
//    /// The document resolving the lab order.
//    pub resolving_document: i64,
//
//    /// The state of the resolution (outstanding, fulfilled, cancelled).
//    pub state: Resol,
//
//    /// The note attached to the resolution (optional).
//    pub note: Option<String>,
//
//    /// The date the resolution was created.
//    pub created_date: OffsetDateTime,
//
//    /// The date the resolution was deleted (optional).
//    pub deleted_date: Option<OffsetDateTime>,
//}

/// Represents the data required to create a new lab order.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderForCreate {
    // Add fields relevant for creating a lab order.
    pub patient: i64,
    pub ordering_physician: i64,
    pub practice: i64,
    pub vendor: i64,
}

/// Represents the data required to update an existing lab order.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LabOrderForUpdate {
    pub resolution: Option<Resolution>,
    pub signed_by: Option<i64>,
    pub signed_date: Option<OffsetDateTime>,
    // Add other fields for updating the lab order.
}

impl Resource for LabOrder {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/lab_orders"
    }
}
