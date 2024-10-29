use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderTest {
    /// The unique identifier of the Lab Order Test.
    pub id: i64,

    /// Any code associated with the test, usually provided by the lab vendor's compendium.
    pub code: String,

    /// The compendium ID.
    pub compendium: i64,

    /// A list of CPT codes associated with the lab order test.
    #[serde(default)]
    pub cpts: Vec<String>,

    /// The date when the Lab Order Test was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<OffsetDateTime>,

    /// The date when the Lab Order Test was deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<OffsetDateTime>,

    /// The lab vendor ID.
    pub lab_vendor: i64,

    /// The name of the Lab Order Test.
    pub name: String,

    /// The ID of the practice that created the test if practice-created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_created: Option<i64>,

    /// The procedure class of the test.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_class: Option<String>,

    /// A list of Ask on Entry (AOE) Questions that need to be answered by the orderer when creating the order.
    #[serde(default)]
    pub questions: Vec<Question>,

    /// A list of synonyms that are useful when searching for a specific test.
    #[serde(default)]
    pub synonyms: Vec<String>,
}

impl Resource for LabOrderTest {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/lab_order_tests"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    /// The ID of the test associated with the question.
    pub test: i64,

    /// The sequence number of the question.
    pub sequence: i32,

    /// Indicates whether the question is required.
    pub required: bool,

    /// The question details.
    pub question: QuestionDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionDetails {
    /// The unique identifier of the question.
    pub id: i64,

    /// The code of the question.
    pub code: String,

    /// The text of the question.
    pub value: String,

    /// The type of the question.
    /// Allowed values: `"enum"`, etc.
    pub question_type: String,

    /// A secondary code for the question (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_code: Option<String>,

    /// An example format for the question (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_format: Option<String>,

    /// The maximum length of the answer (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,

    /// A list of choices for the question.
    #[serde(default)]
    pub choices: Vec<Choice>,

    /// The date when the question was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<OffsetDateTime>,

    /// The date when the question was deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<OffsetDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    /// The unique identifier of the choice.
    pub id: i64,

    /// The code of the choice.
    pub code: String,

    /// The value of the choice.
    pub value: String,

    /// The date when the choice was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<OffsetDateTime>,

    /// The date when the choice was deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<OffsetDateTime>,
}

/// Represents the data required to create a new Lab Order Test.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabOrderTestForCreate {
    /// The name of the Lab Order Test.
    pub name: String,

    /// Any code associated with the test.
    pub code: String,

    /// The ID of the practice that created the test.
    pub practice_created: i64,

    /// The compendium ID.
    pub compendium: i64,

    /// The lab vendor ID.
    pub lab_vendor: i64,

    /// A list of questions associated with the test.
    #[serde(default)]
    pub questions: Vec<Question>,

    /// A list of synonyms for the test.
    #[serde(default)]
    pub synonyms: Vec<String>,

    /// A list of CPT codes associated with the lab order test.
    #[serde(default)]
    pub cpts: Vec<String>,
}

/// Represents query parameters for searching lab order tests.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LabOrderTestQueryParams {
    /// The codes of the lab tests you want to find.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Vec<String>>,

    /// The IDs of the lab vendors of the lab tests you want to find.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lab_vendor: Option<Vec<i64>>,

    /// The IDs of practices that created the lab tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_created: Option<Vec<i64>>,

    /// The IDs of practices.
    ///
    /// Note: This field corresponds to `practice_created` in the API and is used for filtering lab tests based on the practice.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "practice_created")]
    pub practice: Option<Vec<i64>>,

    /// The IDs of compendiums.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compendium: Option<Vec<i64>>,
}
