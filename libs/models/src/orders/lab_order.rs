use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::{Date, OffsetDateTime};
use utils::time::Rfc3339;

use crate::resource::Resource;

use super::{Resolution, ResolutionState, StatMethod};

time::serde::format_description!(one_true_date, Date, "[year]-[month]-[day]");

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrder {
    /// The ID of the lab order in Elation's systems.
    pub id: i64,
    /// List of physician IDs who should be cc'd on any reports.
    pub ccs: Vec<i64>,
    /// Date at which this order was first seen in the patient's chart.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// Whether the provider has marked the order as confidential.
    pub confidential: bool,
    /// Represents the content of the lab order report.
    pub content: LabOrderContent,
    /// Time at which Elation created this order.
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// Time at which this order was created in whichever system created it.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// Freetext field communicating how the patient should receive their results.
    pub follow_up_method: Option<String>,
    /// The ID of the physician who created the order.
    pub ordering_physician: i64,
    /// The ID of the patient.
    pub patient: i64,
    /// The ID of the practice.
    pub practice: i64,
    /// The requisition ID of the order.
    pub requisition: Option<i64>,
    /// The resolution state of the lab order.
    pub resolution: Option<Resolution>,
    /// The ID of the physician who signed off on the order.
    pub signed_by: Option<i64>,
    /// The time when the order was signed.
    #[serde_as(as = "Option<Rfc3339>")]
    pub signed_date: Option<OffsetDateTime>,
    /// The site where the patient will have the lab performed.
    pub site: Option<Site>,
    /// Special insurance information, used primarily to support specific electronic lab order integrations.
    pub special_insurance: Option<SpecialInsurance>,
    /// Record of the attempts to submit.
    pub submissions: Vec<LabOrderSubmission>,
    /// Record of any specimens collected when creating the order.
    pub specimens: Vec<LabOrderSpecimen>,
    /// Any tags associated with the lab order.
    pub tags: Vec<CodedDocumentTag>,
    /// The date on which the tests should be performed, or were performed.
    #[serde(with = "one_true_date::option")]
    pub test_date: Option<Date>,
    /// The vendor ID.
    pub vendor: i64,
    /// The printable view URL.
    pub printable_view: Option<String>,
    /// The bill type indicating who is financially responsible for the order.
    pub bill_type: Option<BillType>,
    /// List of answers provided for the Ask on Entry (AOE) questions.
    pub answers: Vec<Answer>,
    /// The facility identifier that represents the facility placing the order.
    pub facility: Option<Facility>,
}

impl Resource for LabOrder {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/lab_orders"
    }
}

/// Represents the content of the lab order report.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderContent {
    /// The ID of the Lab Order Content.
    pub id: i64,
    /// The type of report to get for the order if urgent.
    pub stat_method: Option<StatMethod>,
    /// Notes for patient.
    pub patient_instructions: Option<String>,
    /// Notes for Lab.
    pub test_center_notes: Option<String>,
    /// Instructions for patient and lab on how long they should fast before specimen collection.
    pub fasting_method: Option<FastingMethod>,
    /// Number of times order should be performed on a standing basis.
    pub standing_order_frequency: Option<String>,
    /// Date at which standing order should be canceled.
    #[serde(with = "one_true_date::option")]
    pub standing_order_end_date: Option<Date>,
    /// If lab specimen collection is done onsite, this is the time at which the collection was taken.
    #[serde_as(as = "Option<Rfc3339>")]
    pub collection_datetime: Option<OffsetDateTime>,
    /// ICD-10 diagnosis codes provided along with the order.
    pub icd10_codes: Vec<Icd10Code>,
    /// The list of tests that should be performed by the lab.
    pub tests: Vec<LabOrderTest>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderTest {
    /// The ID of the Lab Order Test.
    pub id: i64,
    /// The name of the Lab Order Test.
    pub name: String,
    /// Any code associated with the test, usually provided by the lab vendor's compendium.
    pub code: Option<String>,
    /// Code provided by specific compendiums that indicate the "class" of test.
    pub procedure_class: Option<String>,
    /// The ID of the practice that created the test if practice created.
    pub practice_created: Option<i64>,
    /// The ID of the lab vendor who provides the lab order test.
    pub lab_vendor: Option<i64>,
    /// The ID of the compendium provided by the lab vendor.
    pub compendium: Option<i64>,
    /// A list of CPT codes associated with the lab order test.
    pub cpts: Vec<String>,
    /// A list of synonyms that are useful when searching for a specific test.
    pub synonyms: Vec<String>,
    /// A list of Ask on Entry (AOE) Questions that need to be answered by the orderer when creating the order.
    pub questions: Vec<AOEQuestion>,
    /// Time at which Elation created this Lab Order Test.
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// Time at which this Lab Order Test was deleted.
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Icd10Code {
    /// The ICD-10 code.
    pub code: String,
    /// Description of the ICD-10 code.
    pub description: Option<String>,
}

//#[serde_as]
//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Resolution {
//    /// Internal ID to represent the resolution state.
//    pub id: i64,
//    /// Will be the same as the ID of the lab order.
//    pub document: i64,
//    /// If the order is fulfilled, will indicate the document referenced.
//    pub resolving_document: Option<i64>,
//    /// The resolution state of the order.
//    pub state: ResolutionState,
//    /// Any note that was added to represent why an order was cancelled.
//    pub note: Option<String>,
//    /// The datetime when this resolution state was created.
//    #[serde_as(as = "Option<Rfc3339>")]
//    pub created_date: Option<OffsetDateTime>,
//    /// The datetime when this resolution was deleted.
//    #[serde_as(as = "Option<Rfc3339>")]
//    pub deleted_date: Option<OffsetDateTime>,
//}

//#[derive(Clone, Debug, Serialize, Deserialize)]
//#[serde(rename_all = "lowercase")]
//pub enum ResolutionState {
//    Outstanding,
//    Fulfilled,
//    Cancelled,
//}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    /// The name of the site.
    pub name: Option<String>,
    /// The street address of the site.
    pub address: Option<String>,
    /// The suite number of the site.
    pub suite: Option<String>,
    /// The city of the site.
    pub city: Option<String>,
    /// The state of the site.
    pub state: Option<String>,
    /// The ZIP code of the site.
    pub zip: Option<String>,
    /// The phone number of the site.
    pub phone: Option<String>,
    /// The fax number of the site.
    pub fax: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecialInsurance {
    /// Code for special insurance.
    pub code: Option<String>,
    /// Name for special insurance.
    pub name: Option<String>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderSubmission {
    /// The state of the electronic order submission.
    pub state: Option<String>,
    /// The time the electronic order was submitted to the lab vendor.
    #[serde_as(as = "Option<Rfc3339>")]
    pub time_submitted: Option<OffsetDateTime>,
    /// The time at which a user acknowledged any error shown to them about the electronic submission.
    #[serde_as(as = "Option<Rfc3339>")]
    pub time_acknowledged: Option<OffsetDateTime>,
    /// Useful error information collected as part of the submission.
    pub state_metadata: Option<String>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderSpecimen {
    /// The lab order test for which the specimen was collected.
    pub test: Option<i64>,
    /// Information about the body site where the specimen was collected.
    pub body_site: Option<BodySite>,
    /// A free-text description of the specimen.
    pub description: Option<String>,
    /// An identifier representing the container used to store the specimen.
    pub container_id: Option<i64>,
    /// Coded Modifiers indicating more information about the body site from which the specimen was collected.
    pub modifiers: Vec<BodySiteModifier>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BodySite {
    /// The HL7 code representing the body site from which a specimen was collected.
    pub code: Option<String>,
    /// A textual description of the body site from which the specimen was collected.
    pub text: Option<String>,
    /// Represents if the body site was removed from the specimen.
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BodySiteModifier {
    /// The HL7 code representing the modifier on the body site from which a specimen was collected.
    pub code: Option<String>,
    /// A textual description of the modifier of the body site from which the specimen was collected.
    pub text: Option<String>,
    /// The cardinal position in which the modifiers should be listed.
    pub sequence: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodedDocumentTag {
    /// The ID of the tag.
    pub id: i64,
    /// The name of the tag.
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BillType {
    #[serde(rename = "thirdparty")]
    ThirdParty,
    #[serde(rename = "patient")]
    Patient,
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "workerscomp")]
    WorkersComp,
    #[serde(rename = "specialbilling")]
    SpecialBilling,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Answer {
    /// The ID of the test for which this answer pertains.
    pub test: i64,
    /// The ID of the AOE question for which this answer pertains.
    pub question: i64,
    /// The value entered by free text or chosen by the ordering physician.
    pub value: String,
    /// When the answer was created.
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_date: Option<OffsetDateTime>,
    /// When the answer was deleted.
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Facility {
    /// An identifier that represents the facility placing the order.
    pub id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FastingMethod {
    Fasting12Hour,
    Fasting8Hours,
    Fasting2Hours,
    FastingNone,
    FastingRandom,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AOEQuestion {
    /// The ID of the question.
    pub id: i64,
    /// The text of the question.
    pub text: String,
    /// The type of question (e.g., text, choice).
    pub question_type: String,
    /// List of choices if the question is of type 'choice'.
    pub choices: Option<Vec<String>>,
}

/// Represents the data required to create or update a lab order (used for both POST and PUT requests).
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderForCreate {
    /// The ID of the patient.
    pub patient: i64,
    /// The ID of the practice.
    pub practice: i64,
    /// The ID of the ordering physician.
    pub ordering_physician: i64,
    /// Date at which this order was first seen in the patient's chart.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// Time at which this order was created in whichever system created it.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// Whether the provider has marked the order as confidential.
    pub confidential: Option<bool>,
    /// Freetext field communicating how the patient should receive their results.
    pub follow_up_method: Option<String>,
    /// The resolution state of the lab order.
    pub resolution: Option<ResolutionForCreate>,
    /// The date on which the tests should be performed, or were performed.
    #[serde(with = "one_true_date::option")]
    pub test_date: Option<Date>,
    /// The vendor ID.
    pub vendor: Option<i64>,
    /// The content of the lab order.
    pub content: Option<LabOrderContentForCreate>,
    /// List of physician IDs who should be cc'd on any reports.
    pub ccs: Option<Vec<i64>>,
    /// The bill type indicating who is financially responsible for the order.
    pub bill_type: Option<BillType>,
    /// List of answers provided for the Ask on Entry (AOE) questions.
    pub answers: Option<Vec<AnswerForCreate>>,
    /// The site where the patient will have the lab performed.
    pub site: Option<i64>,
    /// Any tags associated with the lab order.
    pub tags: Option<Vec<i64>>,
}

/// Represents the data required to partially update a lab order (used for PATCH requests).
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LabOrderForUpdate {
    /// The ID of the patient (cannot be changed on update).
    pub patient: Option<i64>,
    /// The ID of the practice (cannot be changed on update).
    pub practice: Option<i64>,
    /// The ID of the ordering physician.
    pub ordering_physician: Option<i64>,
    /// Date at which this order was first seen in the patient's chart.
    #[serde_as(as = "Option<Rfc3339>")]
    pub chart_date: Option<OffsetDateTime>,
    /// Time at which this order was created in whichever system created it.
    #[serde_as(as = "Option<Rfc3339>")]
    pub document_date: Option<OffsetDateTime>,
    /// Whether the provider has marked the order as confidential.
    pub confidential: Option<bool>,
    /// Freetext field communicating how the patient should receive their results.
    pub follow_up_method: Option<String>,
    /// The resolution state of the lab order.
    pub resolution: Option<ResolutionForUpdate>,
    /// The date on which the tests should be performed, or were performed.
    #[serde(with = "one_true_date::option")]
    pub test_date: Option<Date>,
    /// The vendor ID.
    pub vendor: Option<i64>,
    /// The content of the lab order.
    pub content: Option<LabOrderContentForUpdate>,
    /// List of physician IDs who should be cc'd on any reports.
    pub ccs: Option<Vec<i64>>,
    /// The bill type indicating who is financially responsible for the order.
    pub bill_type: Option<BillType>,
    /// List of answers provided for the Ask on Entry (AOE) questions.
    pub answers: Option<Vec<AnswerForCreate>>,
    /// The site where the patient will have the lab performed.
    pub site: Option<i64>,
    /// Any tags associated with the lab order.
    pub tags: Option<Vec<i64>>,
}

/// Represents the content required to create or update a lab order.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderContentForCreate {
    /// The list of tests that should be performed by the lab.
    pub tests: Vec<LabOrderTestForCreate>,
    /// The type of report to get for the order if urgent.
    pub stat_method: Option<StatMethod>,
    /// Notes for patient.
    pub patient_instructions: Option<String>,
    /// Notes for Lab.
    pub test_center_notes: Option<String>,
    /// Instructions for patient and lab on how long they should fast before specimen collection.
    pub fasting_method: Option<FastingMethod>,
    /// Number of times order should be performed on a standing basis.
    pub standing_order_frequency: Option<String>,
    /// Date at which standing order should be canceled.
    #[serde(with = "one_true_date::option")]
    pub standing_order_end_date: Option<Date>,
    /// If lab specimen collection is done onsite, this is the time at which the collection was taken.
    #[serde_as(as = "Option<Rfc3339>")]
    pub collection_datetime: Option<OffsetDateTime>,
    /// ICD-10 diagnosis codes provided along with the order.
    pub icd10_codes: Option<Vec<Icd10Code>>,
}

/// Represents the content required to partially update a lab order.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LabOrderContentForUpdate {
    /// The list of tests that should be performed by the lab.
    pub tests: Option<Vec<LabOrderTestForCreate>>,
    /// The type of report to get for the order if urgent.
    pub stat_method: Option<StatMethod>,
    /// Notes for patient.
    pub patient_instructions: Option<String>,
    /// Notes for Lab.
    pub test_center_notes: Option<String>,
    /// Instructions for patient and lab on how long they should fast before specimen collection.
    pub fasting_method: Option<FastingMethod>,
    /// Number of times order should be performed on a standing basis.
    pub standing_order_frequency: Option<String>,
    /// Date at which standing order should be canceled.
    #[serde(with = "one_true_date::option")]
    pub standing_order_end_date: Option<Date>,
    /// If lab specimen collection is done onsite, this is the time at which the collection was taken.
    #[serde_as(as = "Option<Rfc3339>")]
    pub collection_datetime: Option<OffsetDateTime>,
    /// ICD-10 diagnosis codes provided along with the order.
    pub icd10_codes: Option<Vec<Icd10Code>>,
}

/// Represents a lab order test for creation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderTestForCreate {
    /// The ID of the Lab Order Test.
    pub id: i64,
    // Additional fields can be added if needed.
}

/// Represents the resolution data for creating or updating a lab order.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResolutionForCreate {
    /// The resolution state of the order.
    pub state: ResolutionState,
    /// The document ID (requisition ID number).
    pub resolving_document: Option<i64>,
}

/// Represents the resolution data for partial updates to a lab order.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResolutionForUpdate {
    /// The resolution state of the order.
    pub state: Option<ResolutionState>,
    /// The document ID (requisition ID number).
    pub resolving_document: Option<i64>,
}

/// Represents an answer to an Ask on Entry (AOE) question for creation.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerForCreate {
    /// The ID of the test for which this answer pertains.
    pub test: i64,
    /// The ID of the AOE question for which this answer pertains.
    pub question: i64,
    /// The value entered by free text or chosen by the ordering physician.
    pub value: String,
}

/// Represents query parameters for searching lab orders.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LabOrderQueryParams {
    /// The ID of the patient.
    pub patient: Option<i64>,
    /// The ID of the practice.
    pub practice: Option<i64>,
    /// Whether the lab order is unsigned.
    pub unsigned: Option<bool>,
    /// From signed date (YYYY-MM-DD).
    #[serde(with = "one_true_date::option")]
    pub from_signed_date: Option<Date>,
    /// To signed date (YYYY-MM-DD).
    #[serde(with = "one_true_date::option")]
    pub to_signed_date: Option<Date>,
    /// Document date greater than (YYYY-MM-DD).
    #[serde(rename = "document_date__gt", with = "one_true_date::option")]
    pub document_date_gt: Option<Date>,
    /// Document date greater than or equal to (YYYY-MM-DD).
    #[serde(rename = "document_date__gte", with = "one_true_date::option")]
    pub document_date_gte: Option<Date>,
    /// Document date less than (YYYY-MM-DD).
    #[serde(rename = "document_date__lt", with = "one_true_date::option")]
    pub document_date_lt: Option<Date>,
    /// Document date less than or equal to (YYYY-MM-DD).
    #[serde(rename = "document_date__lte", with = "one_true_date::option")]
    pub document_date_lte: Option<Date>,
}
