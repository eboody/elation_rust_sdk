use lib_utils::time::Rfc3339;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use super::{AbnormalFlag, ReportType, Status};

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grid {
    pub accession_number: String,
    #[serde_as(as = "Rfc3339")]
    pub resulted_date: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub collected_date: OffsetDateTime,
    pub status: Status,
    pub note: Option<String>,
    pub results: Vec<ResultEntry>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResultEntry {
    pub status: Option<Status>,
    pub value: String,
    pub value_type: Option<String>,
    pub text: Option<String>,
    pub note: Option<String>,
    pub reference_min: Option<String>,
    pub reference_max: Option<String>,
    pub units: Option<String>,
    pub is_abnormal: bool,
    pub abnormal_flag: Option<AbnormalFlag>,

    pub test: Option<Test>,
    pub test_category: Option<TestCategory>,
    pub test_code: Option<String>,
    pub test_name: Option<String>,
    pub test_loinc: Option<String>,
    pub test_category_value: Option<String>,
    pub test_category_description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    pub code: Option<String>,
    pub name: Option<String>,
    pub loinc: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCategory {
    pub value: Option<String>,
    pub description: Option<String>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabReport {
    pub id: i64,
    pub patient: i64,
    pub custom_title: Option<String>,
    pub report_type: ReportType,
    pub requisition_number: Option<String>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub reported_date: Option<OffsetDateTime>,
    pub practice: i64,
    pub physician: Option<i64>,
    #[serde_as(as = "Rfc3339")]
    pub document_date: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub chart_date: OffsetDateTime,
    #[serde_as(as = "Option<Rfc3339>")]
    pub signed_date: Option<OffsetDateTime>,
    pub signed_by: Option<i64>,
    #[serde_as(as = "Rfc3339")]
    pub created_date: OffsetDateTime,
    pub vendor: Option<i64>,
    pub printable_view: String,
    #[serde_as(as = "Option<Rfc3339>")]
    pub deleted_date: Option<OffsetDateTime>,
    pub grids: Vec<Grid>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct LabReportQueryParams {
    pub patients: Option<Vec<i64>>,
    pub practice: Option<i32>,
    pub document_date__lt: Option<String>,
    pub document_date__gt: Option<String>,
    pub document_date__lte: Option<String>,
    pub document_date__gte: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}
