use serde::{Deserialize, Serialize};
use super::common::{Validate, ValidationError, validation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "teamId")]
    pub team_id: i64,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    #[serde(rename = "membersCount")]
    pub members_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamList {
    pub teams: Vec<Team>,
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

#[derive(Debug, Default)]
pub struct CreateTeamRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    avatar_id: Option<String>,
}

impl CreateTeamRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn avatar_id(mut self, avatar_id: impl Into<String>) -> Self {
        self.avatar_id = Some(avatar_id.into());
        self
    }

    pub fn build(self) -> Result<CreateTeamRequest, &'static str> {
        let name = self.name.ok_or("name is required")?;

        Ok(CreateTeamRequest {
            team: CreateTeamInner {
                name,
                description: self.description,
            },
            avatar_id: self.avatar_id,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamInner {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub team: CreateTeamInner,
    #[serde(rename = "avatarId")]
    pub avatar_id: Option<String>,
}

impl Validate for CreateTeamRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        // Name: max 40 chars, pattern modified to avoid look-ahead
        validation::validate_max_length(&self.team.name, 40, "name")?;
        lazy_static::lazy_static! {
            static ref NAME_PATTERN: regex::Regex = regex::Regex::new(
                r"^[a-zA-Z\d][-a-zA-Z\d_]{0,38}[a-zA-Z\d]$"
            ).unwrap();
        }
        validation::validate_pattern(&self.team.name, &NAME_PATTERN, "name")?;

        // Optional description: max 250 chars
        if let Some(ref desc) = self.team.description {
            validation::validate_max_length(desc, 250, "description")?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamResponse {
    pub team: Team,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamMemberRequest {
    #[serde(rename = "userNameOrEmail")]
    pub user_name_or_email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamMemberResponse {
    pub member: super::member::Member,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "avatarId")]
    pub avatar_id: Option<String>,
}

impl Validate for UpdateTeamRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        validation::validate_max_length(&self.name, 40, "name")?;
        if let Some(ref desc) = self.description {
            validation::validate_max_length(desc, 250, "description")?;
        }
        Ok(())
    }
} 