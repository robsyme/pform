use serde::{Deserialize, Serialize};
use super::common::{Validate, ValidationError, validation, WorkspaceId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(rename = "id")]
    pub id: WorkspaceId,
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub description: Option<String>,
    pub visibility: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceList {
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WorkspaceVisibility {
    Private,
    Shared,
}

#[derive(Debug, Default)]
pub struct CreateWorkspaceRequestBuilder {
    name: Option<String>,
    full_name: Option<String>,
    description: Option<String>,
    visibility: Option<WorkspaceVisibility>,
}

impl CreateWorkspaceRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn full_name(mut self, full_name: impl Into<String>) -> Self {
        self.full_name = Some(full_name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn visibility(mut self, visibility: WorkspaceVisibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn build(self) -> Result<CreateWorkspaceRequest, &'static str> {
        let name = self.name.ok_or("name is required")?;
        let full_name = self.full_name.ok_or("full_name is required")?;
        let visibility = self.visibility.ok_or("visibility is required")?;

        Ok(CreateWorkspaceRequest {
            workspace: CreateWorkspaceInner {
                id: None,
                name,
                full_name,
                description: self.description,
                visibility,
            },
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkspaceInner {
    pub id: Option<WorkspaceId>,
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub description: Option<String>,
    pub visibility: WorkspaceVisibility,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub workspace: CreateWorkspaceInner,
}

impl Validate for CreateWorkspaceRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        validation::validate_max_length(&self.workspace.name, 40, "name")?;
        validation::validate_max_length(&self.workspace.full_name, 100, "fullName")?;
        if let Some(ref desc) = self.workspace.description {
            validation::validate_max_length(desc, 1000, "description")?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceResponse {
    pub workspace: Workspace,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub description: Option<String>,
    pub visibility: WorkspaceVisibility,
}

impl Validate for UpdateWorkspaceRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        validation::validate_max_length(&self.name, 40, "name")?;
        validation::validate_max_length(&self.full_name, 100, "fullName")?;
        if let Some(ref desc) = self.description {
            validation::validate_max_length(desc, 1000, "description")?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddWorkspaceParticipantRequest {
    #[serde(rename = "memberId", skip_serializing_if = "Option::is_none")]
    pub member_id: Option<i64>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
    #[serde(rename = "userNameOrEmail", skip_serializing_if = "Option::is_none")]
    pub user_name_or_email: Option<String>,
}

impl AddWorkspaceParticipantRequest {
    pub fn with_member_id(member_id: i64) -> Self {
        Self {
            member_id: Some(member_id),
            team_id: None,
            user_name_or_email: None,
        }
    }

    pub fn with_team_id(team_id: i64) -> Self {
        Self {
            member_id: None,
            team_id: Some(team_id),
            user_name_or_email: None,
        }
    }

    pub fn with_user_name_or_email(user_name_or_email: impl Into<String>) -> Self {
        Self {
            member_id: None,
            team_id: None,
            user_name_or_email: Some(user_name_or_email.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListWorkspacesResponse {
    pub workspaces: Vec<Workspace>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_name_validation() {
        let request = CreateWorkspaceRequest {
            workspace: CreateWorkspaceInner {
                id: None,
                name: "a".repeat(41),
                full_name: "Test Workspace".to_string(),
                description: None,
                visibility: WorkspaceVisibility::Private,
            },
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_valid_workspace_request() {
        let request = CreateWorkspaceRequest {
            workspace: CreateWorkspaceInner {
                id: None,
                name: "test-workspace".to_string(),
                full_name: "Test Workspace".to_string(),
                description: Some("Description".to_string()),
                visibility: WorkspaceVisibility::Private,
            },
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_description_length() {
        let request = CreateWorkspaceRequest {
            workspace: CreateWorkspaceInner {
                id: None,
                name: "test".to_string(),
                full_name: "Test".to_string(),
                description: Some("a".repeat(1001)),
                visibility: WorkspaceVisibility::Private,
            },
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let request = CreateWorkspaceRequestBuilder::new()
            .name("test-workspace")
            .full_name("Test Workspace")
            .description("Description")
            .visibility(WorkspaceVisibility::Private)
            .build();
        assert!(request.is_ok());

        let request = request.unwrap();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_workspace_builder() {
        let request = CreateWorkspaceRequestBuilder::new()
            .name("test-workspace")
            .full_name("Test Workspace")
            .description("A test workspace")
            .visibility(WorkspaceVisibility::Private)
            .build()
            .unwrap();

        assert_eq!(request.workspace.name, "test-workspace");
        assert_eq!(request.workspace.full_name, "Test Workspace");
        assert_eq!(request.workspace.description.unwrap(), "A test workspace");
        assert!(matches!(request.workspace.visibility, WorkspaceVisibility::Private));
    }
} 