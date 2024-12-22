use serde::{Deserialize, Serialize};
use super::common::{OrgRole, WspRole, ParticipantType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "memberId")]
    pub member_id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub avatar: Option<String>,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberList {
    pub members: Vec<Member>,
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    #[serde(rename = "memberId")]
    pub member_id: i64,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "orgRole")]
    pub org_role: OrgRole,
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "wspRole")]
    pub wsp_role: WspRole,
    #[serde(rename = "type")]
    pub participant_type: ParticipantType,
    #[serde(rename = "teamAvatarUrl")]
    pub team_avatar_url: String,
    #[serde(rename = "userAvatarUrl")]
    pub user_avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantList {
    pub participants: Vec<Participant>,
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrganizationMemberRequest {
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMemberRoleRequest {
    pub role: OrgRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantResponse {
    pub participant: Participant,
} 