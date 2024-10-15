use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::resource::Resource;

/// Represents the Patient Provider Team, which includes a collection of providers assisting in the care of a patient.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientProviderTeam {
    /// The ID of the patient provider team.
    pub patient_provider_team_id: i64,

    /// The ID of the patient.
    pub patient_id: i64,

    /// The list of team members providing care to the patient.
    pub team_members: Vec<PatientProviderTeamMember>,
}

/// Represents a team member in the patient provider team.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientProviderTeamMember {
    /// The ID of the patient provider team.
    pub patient_provider_team_id: i64,

    /// The ID of the patient provider team member.
    pub patient_provider_team_member_id: i64,

    /// The ID of the patient.
    pub patient_id: i64,

    /// The ID of the physician.
    pub physician_id: i64,

    /// The group the team member belongs to (main or other).
    pub group: String,

    /// The rank of the team member (for sorting).
    pub rank: i32,

    /// The reason for the team member's involvement in treatment.
    pub treatment_reason: Option<String>,

    /// The earliest known interaction between the team member and the patient.
    pub earliest_activity: Option<OffsetDateTime>,

    /// The latest known interaction between the team member and the patient.
    pub latest_activity: Option<OffsetDateTime>,

    /// The last known time when the team member's activity summary was refreshed.
    pub activity_summary_last_refreshed: Option<OffsetDateTime>,
}

/// Represents the data required to create a new Patient Provider Team.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientProviderTeamForCreate {
    /// The ID of the patient.
    pub patient_id: i64,

    /// The list of team members providing care to the patient.
    pub team_members: Vec<PatientProviderTeamMemberForCreate>,
}

/// Represents the data required to create a new Patient Provider Team member.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientProviderTeamMemberForCreate {
    /// The ID of the physician.
    pub physician_id: i64,

    /// The group the team member belongs to (main or other).
    pub group: String,

    /// The rank of the team member (for sorting).
    pub rank: i32,

    /// The reason for the team member's involvement in treatment (optional).
    pub treatment_reason: Option<String>,
}

/// Represents the data required to update an existing Patient Provider Team.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PatientProviderTeamForUpdate {
    /// The list of team members providing care to the patient (optional).
    pub team_members: Option<Vec<PatientProviderTeamMemberForUpdate>>,
}

/// Represents the data required to update an existing Patient Provider Team member.
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PatientProviderTeamMemberForUpdate {
    /// The ID of the physician (optional).
    pub physician_id: Option<i64>,

    /// The group the team member belongs to (main or other) (optional).
    pub group: Option<String>,

    /// The rank of the team member (for sorting) (optional).
    pub rank: Option<i32>,

    /// The reason for the team member's involvement in treatment (optional).
    pub treatment_reason: Option<String>,
}

impl Resource for PatientProviderTeam {
    type Id = i64;

    fn endpoint() -> &'static str {
        "/patient_provider_teams"
    }
}
