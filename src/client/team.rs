use crate::errors::SeqeraError;
use crate::models::team::*;
use crate::models::member::Member;
use crate::models::common::Validate;
use crate::models::workspace::WorkspaceList;

impl super::SeqeraClient {
    pub async fn list_organization_teams(&self, org_id: i64) -> Result<TeamList, SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/teams", org_id))?;
        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn validate_team_name(&self, org_id: i64, name: &str) -> Result<(), SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/teams/validate", org_id))?;
        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .query(&[("name", name)])
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn create_team(
        &self,
        org_id: i64,
        request: CreateTeamRequest,
    ) -> Result<Team, SeqeraError> {
        request.validate()?;
        let url = self.base_url.join(&format!("orgs/{}/teams", org_id))?;
        let request = self
            .client
            .post(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: TeamResponse = response.json().await?;
        Ok(wrapper.team)
    }

    pub async fn create_team_member(
        &self,
        org_id: i64,
        team_id: i64,
        request: CreateTeamMemberRequest,
    ) -> Result<Member, SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/teams/{}/members", org_id, team_id))?;
        let request = self
            .client
            .post(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: TeamMemberResponse = response.json().await?;
        Ok(wrapper.member)
    }

    pub async fn get_team(&self, org_id: i64, team_id: i64) -> Result<Team, SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/teams/{}", org_id, team_id))?;
        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: TeamResponse = response.json().await?;
        Ok(wrapper.team)
    }

    pub async fn list_team_workspaces(
        &self,
        org_id: i64,
        team_id: i64,
        max: Option<i32>,
        offset: Option<i32>,
        search: Option<&str>,
    ) -> Result<WorkspaceList, SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/teams/{}/workspaces", org_id, team_id))?;
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

    pub async fn update_team(
        &self,
        org_id: i64,
        team_id: i64,
        request: UpdateTeamRequest,
    ) -> Result<(), SeqeraError> {
        request.validate()?;
        let url = self.base_url.join(&format!("orgs/{}/teams/{}", org_id, team_id))?;
        let request = self.client
            .put(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn delete_team_member(
        &self,
        org_id: i64,
        team_id: i64,
        member_id: i64,
    ) -> Result<(), SeqeraError> {
        let url = self.base_url.join(&format!(
            "orgs/{}/teams/{}/members/{}/delete",
            org_id, team_id, member_id
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
    use crate::models::common::WorkspaceId;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_team() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orgs/123/teams/456"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "team": {
                    "teamId": 456,
                    "name": "test-team",
                    "description": "Test Team Description",
                    "avatarUrl": "https://example.com/avatar.png",
                    "membersCount": 5
                }
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let team = client.get_team(123, 456).await.unwrap();
        assert_eq!(team.team_id, 456);
        assert_eq!(team.name, "test-team");
        assert_eq!(team.description.unwrap(), "Test Team Description");
        assert_eq!(team.avatar_url.unwrap(), "https://example.com/avatar.png");
        assert_eq!(team.members_count, 5);
    }

    #[tokio::test]
    async fn test_list_team_workspaces() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orgs/123/teams/456/workspaces"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "workspaces": [{
                    "id": 789,
                    "name": "test-workspace",
                    "fullName": "Test Workspace",
                    "description": "Test Description",
                    "visibility": "PRIVATE"
                }]
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let workspaces = client.list_team_workspaces(123, 456, None, None, None).await.unwrap();
        assert_eq!(workspaces.workspaces.len(), 1);
        let workspace = &workspaces.workspaces[0];
        assert_eq!(workspace.id, WorkspaceId(789));
        assert_eq!(workspace.name, "test-workspace");
        assert_eq!(workspace.full_name, "Test Workspace");
        assert_eq!(workspace.description.as_ref().unwrap(), "Test Description");
        assert_eq!(workspace.visibility, "PRIVATE");
    }

    #[tokio::test]
    async fn test_update_team() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/orgs/123/teams/456"))
            .and(header("authorization", "Bearer test-token"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let request = UpdateTeamRequest {
            name: "updated-team".to_string(),
            description: Some("Updated team description".to_string()),
            avatar_id: Some("new-avatar-123".to_string()),
        };

        let result = client.update_team(123, 456, request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_team_member() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/orgs/123/teams/456/members/789/delete"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.delete_team_member(123, 456, 789).await;
        assert!(result.is_ok());
    }
} 