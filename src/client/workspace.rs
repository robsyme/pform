use crate::errors::SeqeraError;
use crate::models::workspace::*;
use crate::models::member::ParticipantList;
use crate::models::common::{Validate, OrgId, WorkspaceId};
use crate::models::member::{Participant, ParticipantResponse};

impl super::SeqeraClient {
    pub async fn get_workspace(&self, org_id: impl Into<OrgId>, workspace_id: impl Into<WorkspaceId>) -> Result<Workspace, SeqeraError> {
        let org_id = org_id.into();
        let workspace_id = workspace_id.into();
        let url = self.base_url.join(&format!("orgs/{}/workspaces/{}", org_id.0, workspace_id.0))?;
        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn list_workspaces(
        &self,
        org_id: impl Into<OrgId>,
    ) -> Result<ListWorkspacesResponse, SeqeraError> {
        let org_id = org_id.into();
        let url = self.base_url.join(&format!("orgs/{}/workspaces", org_id.0))?;

        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn create_workspace(
        &self,
        org_id: impl Into<OrgId>,
        request: CreateWorkspaceRequest,
    ) -> Result<Workspace, SeqeraError> {
        let org_id = org_id.into();
        request.validate()?;
        let url = self.base_url.join(&format!("orgs/{}/workspaces", org_id.0))?;
        let request = self
            .client
            .post(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: WorkspaceResponse = response.json().await?;
        Ok(wrapper.workspace)
    }

    pub async fn validate_workspace_name(&self, org_id: impl Into<OrgId>, name: &str) -> Result<(), SeqeraError> {
        let org_id = org_id.into();
        let url = self.base_url.join(&format!("orgs/{}/workspaces/validate", org_id.0))?;
        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .query(&[("name", name)])
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn list_workspace_participants(
        &self,
        org_id: impl Into<OrgId>,
        workspace_id: impl Into<WorkspaceId>,
        max: Option<i32>,
        offset: Option<i32>,
        search: Option<&str>,
    ) -> Result<ParticipantList, SeqeraError> {
        let org_id = org_id.into();
        let workspace_id = workspace_id.into();
        let url = self.base_url.join(&format!("orgs/{}/workspaces/{}/participants", org_id.0, workspace_id.0))?;
        let mut request = self.client
            .get(url)
            .header("Authorization", self.auth_header());

        if let Some(max) = max {
            request = request.query(&[("max", max)]);
        }
        if let Some(offset) = offset {
            request = request.query(&[("offset", offset)]);
        }
        if let Some(search) = search {
            request = request.query(&[("search", search)]);
        }

        let request = request.build()?;
        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn update_workspace(
        &self,
        org_id: impl Into<OrgId>,
        workspace_id: impl Into<WorkspaceId>,
        request: UpdateWorkspaceRequest,
    ) -> Result<Workspace, SeqeraError> {
        let org_id = org_id.into();
        let workspace_id = workspace_id.into();
        request.validate()?;
        let url = self.base_url.join(&format!("orgs/{}/workspaces/{}", org_id.0, workspace_id.0))?;
        let request = self.client
            .put(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: WorkspaceResponse = response.json().await?;
        Ok(wrapper.workspace)
    }

    pub async fn add_workspace_participant(
        &self,
        org_id: impl Into<OrgId>,
        workspace_id: impl Into<WorkspaceId>,
        request: AddWorkspaceParticipantRequest,
    ) -> Result<Participant, SeqeraError> {
        let org_id = org_id.into();
        let workspace_id = workspace_id.into();
        let url = self.base_url.join(&format!(
            "orgs/{}/workspaces/{}/participants/add",
            org_id.0, workspace_id.0
        ))?;
        let request = self.client
            .put(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: ParticipantResponse = response.json().await?;
        Ok(wrapper.participant)
    }

    pub async fn delete_workspace(
        &self,
        org_id: impl Into<OrgId>,
        workspace_id: impl Into<WorkspaceId>,
    ) -> Result<(), SeqeraError> {
        let org_id = org_id.into();
        let workspace_id = workspace_id.into();
        let url = self.base_url.join(&format!("orgs/{}/workspaces/{}", org_id.0, workspace_id.0))?;
        let request = self.client
            .delete(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn leave_workspace(
        &self,
        org_id: impl Into<OrgId>,
        workspace_id: impl Into<WorkspaceId>,
    ) -> Result<(), SeqeraError> {
        let org_id = org_id.into();
        let workspace_id = workspace_id.into();
        let url = self.base_url.join(&format!(
            "orgs/{}/workspaces/{}/participants",
            org_id.0, workspace_id.0
        ))?;
        let request = self.client
            .delete(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn delete_workspace_participant(
        &self,
        org_id: impl Into<OrgId>,
        workspace_id: impl Into<WorkspaceId>,
        participant_id: i64,
    ) -> Result<(), SeqeraError> {
        let org_id = org_id.into();
        let workspace_id = workspace_id.into();
        let url = self.base_url.join(&format!(
            "orgs/{}/workspaces/{}/participants/{}",
            org_id.0, workspace_id.0, participant_id
        ))?;
        let request = self.client
            .delete(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header, query_param};
    use crate::models::common::{OrgRole, WspRole, ParticipantType};
    use serde_json::json;

    #[tokio::test]
    async fn test_validate_workspace_name() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orgs/123/workspaces/validate"))
            .and(query_param("name", "test-workspace"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.validate_workspace_name(123, "test-workspace").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_workspace_name_duplicate() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orgs/123/workspaces/validate"))
            .and(query_param("name", "existing-workspace"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(409)
                .set_body_json(json!({"message": "Workspace name already exists"})))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.validate_workspace_name(123, "existing-workspace").await;
        assert!(result.is_err());
        if let Err(SeqeraError::Api { status, .. }) = result {
            assert_eq!(status, reqwest::StatusCode::CONFLICT);
        } else {
            panic!("Expected API error with conflict status");
        }
    }

    #[tokio::test]
    async fn test_list_workspace_participants() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orgs/123/workspaces/456/participants"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "participants": [{
                    "participantId": 789,
                    "memberId": 101,
                    "userName": "test-user",
                    "firstName": "Test",
                    "lastName": "User",
                    "email": "test@example.com",
                    "orgRole": "member",
                    "teamId": 201,
                    "teamName": "Test Team",
                    "wspRole": "admin",
                    "type": "MEMBER",
                    "teamAvatarUrl": "https://example.com/team-avatar.png",
                    "userAvatarUrl": "https://example.com/user-avatar.png"
                }],
                "totalSize": 1
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.list_workspace_participants(123, 456, None, None, None).await.unwrap();
        assert_eq!(result.total_size, 1);
        assert_eq!(result.participants.len(), 1);

        let participant = &result.participants[0];
        assert_eq!(participant.participant_id, 789);
        assert_eq!(participant.member_id, 101);
        assert_eq!(participant.user_name, "test-user");
        assert_eq!(participant.first_name, "Test");
        assert_eq!(participant.last_name, "User");
        assert_eq!(participant.email, "test@example.com");
        assert_eq!(participant.org_role, OrgRole::Member);
        assert_eq!(participant.team_id, 201);
        assert_eq!(participant.team_name, "Test Team");
        assert_eq!(participant.wsp_role, WspRole::Admin);
        assert_eq!(participant.participant_type, ParticipantType::Member);
        assert_eq!(participant.team_avatar_url, "https://example.com/team-avatar.png");
        assert_eq!(participant.user_avatar_url, "https://example.com/user-avatar.png");
    }

    #[tokio::test]
    async fn test_update_workspace() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/orgs/123/workspaces/456"))
            .and(header("authorization", "Bearer test-token"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "workspace": {
                    "id": 456,
                    "name": "updated-workspace",
                    "fullName": "Updated Workspace",
                    "description": "Updated workspace description",
                    "visibility": "PRIVATE",
                    "dateCreated": "2023-01-01T00:00:00Z",
                    "lastUpdated": "2023-01-02T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let request = UpdateWorkspaceRequest {
            name: "updated-workspace".to_string(),
            full_name: "Updated Workspace".to_string(),
            description: Some("Updated workspace description".to_string()),
            visibility: WorkspaceVisibility::Private,
        };

        let workspace = client.update_workspace(123, 456, request).await.unwrap();
        assert_eq!(workspace.id, WorkspaceId(456));
        assert_eq!(workspace.name, "updated-workspace");
        assert_eq!(workspace.full_name, "Updated Workspace");
        assert_eq!(workspace.description.unwrap(), "Updated workspace description");
        assert_eq!(workspace.visibility, "PRIVATE");
    }

    #[tokio::test]
    async fn test_add_workspace_participant() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/orgs/123/workspaces/456/participants/add"))
            .and(header("authorization", "Bearer test-token"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "participant": {
                    "participantId": 789,
                    "memberId": 101,
                    "userName": "test-user",
                    "firstName": "Test",
                    "lastName": "User",
                    "email": "test@example.com",
                    "orgRole": "member",
                    "teamId": 201,
                    "teamName": "Test Team",
                    "wspRole": "admin",
                    "type": "MEMBER",
                    "teamAvatarUrl": "https://example.com/team-avatar.png",
                    "userAvatarUrl": "https://example.com/user-avatar.png"
                }
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let request = AddWorkspaceParticipantRequest::with_user_name_or_email("test@example.com");
        let participant = client.add_workspace_participant(123, 456, request).await.unwrap();

        assert_eq!(participant.participant_id, 789);
        assert_eq!(participant.member_id, 101);
        assert_eq!(participant.user_name, "test-user");
        assert_eq!(participant.first_name, "Test");
        assert_eq!(participant.last_name, "User");
        assert_eq!(participant.email, "test@example.com");
        assert_eq!(participant.org_role, OrgRole::Member);
        assert_eq!(participant.team_id, 201);
        assert_eq!(participant.team_name, "Test Team");
        assert_eq!(participant.wsp_role, WspRole::Admin);
        assert_eq!(participant.participant_type, ParticipantType::Member);
        assert_eq!(participant.team_avatar_url, "https://example.com/team-avatar.png");
        assert_eq!(participant.user_avatar_url, "https://example.com/user-avatar.png");
    }

    #[tokio::test]
    async fn test_delete_workspace() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/orgs/123/workspaces/456"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.delete_workspace(123, 456).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_leave_workspace() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/orgs/123/workspaces/456/participants"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.leave_workspace(123, 456).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_workspace_participant() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/orgs/123/workspaces/456/participants/789"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.delete_workspace_participant(123, 456, 789).await;
        assert!(result.is_ok());
    }
} 