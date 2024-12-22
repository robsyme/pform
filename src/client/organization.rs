use crate::errors::SeqeraError;
use crate::models::organization::*;
use crate::models::member::{Member, AddOrganizationMemberRequest, UpdateMemberRoleRequest};
use crate::models::common::{Validate, OrgRole};

impl super::SeqeraClient {
    pub async fn list_organizations(&self) -> Result<ListOrganizationsResponse, SeqeraError> {
        let url = self.base_url.join("orgs")?;
        let request = self
            .client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn get_organization(&self, org_id: i64) -> Result<Organization, SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}", org_id))?;
        let request = self
            .client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: OrganizationResponse = response.json().await?;
        Ok(wrapper.organization)
    }

    pub async fn validate_organization_name(&self, name: &str) -> Result<(), SeqeraError> {
        let url = self.base_url.join("orgs/validate")?;
        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .query(&[("name", name)])
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, SeqeraError> {
        request.validate()?;
        let url = self.base_url.join("orgs")?;
        let request = self
            .client
            .post(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: OrganizationResponse = response.json().await?;
        Ok(wrapper.organization)
    }

    pub async fn update_organization(
        &self,
        org_id: i64,
        update: UpdateOrganizationRequest,
    ) -> Result<Organization, SeqeraError> {
        update.validate()?;
        let url = self.base_url.join(&format!("orgs/{}", org_id))?;
        let request = self
            .client
            .put(url)
            .header("Authorization", self.auth_header())
            .json(&update)
            .build()?;

        let response = self.handle_response(request).await?;
        let wrapper: OrganizationResponse = response.json().await?;
        Ok(wrapper.organization)
    }

    pub async fn delete_organization(&self, org_id: i64) -> Result<(), SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}", org_id))?;
        let request = self
            .client
            .delete(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn find_organization_by_name(&self, name: &str) -> Result<Option<Organization>, SeqeraError> {
        let orgs = self.list_organizations().await?;
        Ok(orgs.organizations.into_iter().find(|org| org.name == name))
    }

    pub async fn get_organization_quotas(
        &self,
        org_id: i64,
        include: Option<Vec<String>>,
    ) -> Result<OrganizationQuotas, SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/quotas", org_id))?;
        let mut request = self.client
            .get(url)
            .header("Authorization", self.auth_header());

        if let Some(include) = include {
            request = request.query(&[("include", include.join(","))]);
        }

        let request = request.build()?;
        let response = self.handle_response(request).await?;
        let wrapper: OrganizationQuotasResponse = response.json().await?;
        Ok(wrapper.quotas)
    }

    pub async fn add_organization_member(
        &self,
        org_id: i64,
        user: impl Into<String>,
    ) -> Result<Member, SeqeraError> {
        let request = AddOrganizationMemberRequest {
            user: user.into(),
        };

        let url = self.base_url.join(&format!("orgs/{}/members/add", org_id))?;
        let request = self.client
            .put(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn update_member_role(
        &self,
        org_id: i64,
        member_id: i64,
        role: OrgRole,
    ) -> Result<(), SeqeraError> {
        let request = UpdateMemberRoleRequest { role };
        let url = self.base_url.join(&format!("orgs/{}/members/{}/role", org_id, member_id))?;
        let request = self.client
            .put(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn leave_organization(&self, org_id: i64) -> Result<(), SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/members/leave", org_id))?;
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
    use wiremock::matchers::{method, path, header};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_organization_quotas() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orgs/123/quotas"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "quotas": {
                    "maxWorkspaces": 10,
                    "maxMembers": 20,
                    "maxTeams": 5,
                    "maxPipelinesPerWorkspace": 100,
                    "maxParticipantsPerWorkspace": 50,
                    "maxDatasetsPerWorkspace": 30,
                    "maxVersionsPerDataset": 10,
                    "maxRuns": 1000,
                    "maxLabelsPerWorkspace": 100,
                    "maxDataStudiosRunning": 5
                }
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let quotas = client.get_organization_quotas(123, None).await.unwrap();
        assert_eq!(quotas.max_workspaces, 10);
        assert_eq!(quotas.max_members, 20);
        assert_eq!(quotas.max_teams, 5);
        assert_eq!(quotas.max_pipelines_per_workspace, 100);
        assert_eq!(quotas.max_participants_per_workspace, 50);
        assert_eq!(quotas.max_datasets_per_workspace, 30);
        assert_eq!(quotas.max_versions_per_dataset, 10);
        assert_eq!(quotas.max_runs, 1000);
        assert_eq!(quotas.max_labels_per_workspace, 100);
        assert_eq!(quotas.max_data_studios_running, 5);
    }

    #[tokio::test]
    async fn test_add_organization_member() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/orgs/123/members/add"))
            .and(header("authorization", "Bearer test-token"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "memberId": 456,
                "userId": 789,
                "userName": "test-user",
                "email": "test@example.com",
                "firstName": "Test",
                "lastName": "User",
                "avatar": "https://example.com/avatar.png",
                "role": "member"
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let member = client.add_organization_member(123, "test@example.com").await.unwrap();
        assert_eq!(member.member_id, 456);
        assert_eq!(member.user_id, 789);
        assert_eq!(member.user_name, "test-user");
        assert_eq!(member.email, "test@example.com");
        assert_eq!(member.first_name, "Test");
        assert_eq!(member.last_name, "User");
        assert_eq!(member.avatar.unwrap(), "https://example.com/avatar.png");
        assert_eq!(member.role, "member");
    }

    #[tokio::test]
    async fn test_update_member_role() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/orgs/123/members/456/role"))
            .and(header("authorization", "Bearer test-token"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.update_member_role(123, 456, OrgRole::Member).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_leave_organization() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/orgs/123/members/leave"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let result = client.leave_organization(123).await;
        assert!(result.is_ok());
    }
} 