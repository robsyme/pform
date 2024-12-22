use crate::errors::SeqeraError;
use crate::models::common::WorkspaceId;
use crate::models::compute_env::{
    ComputeEnvDetails, ComputeEnvStatus, GetComputeEnvResponse, ListComputeEnvsResponse,
    UpdateComputeEnvRequest,
};

impl super::SeqeraClient {
    pub async fn list_compute_envs(
        &self,
        workspace_id: impl Into<WorkspaceId>,
        status: Option<ComputeEnvStatus>,
    ) -> Result<ListComputeEnvsResponse, SeqeraError> {
        let mut url = self.base_url.join("compute-envs")?;

        let mut query_pairs = url.query_pairs_mut();
        query_pairs.append_pair("workspaceId", &workspace_id.into().0.to_string());
        if let Some(status) = status {
            query_pairs.append_pair("status", &status.to_string());
        }
        drop(query_pairs);

        let response = self
            .client
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn validate_compute_env_name(
        &self,
        workspace_id: impl Into<WorkspaceId>,
        name: &str,
    ) -> Result<(), SeqeraError> {
        let mut url = self.base_url.join("compute-envs/validate")?;
        
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.append_pair("workspaceId", &workspace_id.into().0.to_string());
        query_pairs.append_pair("name", name);
        drop(query_pairs);

        let request = self
            .client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let _ = self.handle_response(request).await?;
        Ok(())
    }

    pub async fn get_compute_env(
        &self,
        compute_env_id: impl AsRef<str>,
        workspace_id: impl Into<WorkspaceId>,
    ) -> Result<ComputeEnvDetails, SeqeraError> {
        let mut url = self
            .base_url
            .join(&format!("compute-envs/{}", compute_env_id.as_ref()))?;
        
        url.query_pairs_mut()
            .append_pair("workspaceId", &workspace_id.into().0.to_string());

        let response = self
            .client
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        let wrapper: GetComputeEnvResponse = response.json().await?;
        Ok(wrapper.compute_env)
    }

    pub async fn update_compute_env(
        &self,
        compute_env_id: impl AsRef<str>,
        workspace_id: impl Into<WorkspaceId>,
        request: UpdateComputeEnvRequest,
    ) -> Result<(), SeqeraError> {
        let mut url = self
            .base_url
            .join(&format!("compute-envs/{}", compute_env_id.as_ref()))?;
            
        url.query_pairs_mut()
            .append_pair("workspaceId", &workspace_id.into().0.to_string());

        self.client
            .put(url)
            .header("Authorization", self.auth_header())
            .json(&request)
            .send()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_compute_envs() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/compute-envs"))
            .and(query_param("workspaceId", "123"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "computeEnvs": [{
                    "id": "ce-1",
                    "name": "test-env",
                    "platform": "aws-batch", 
                    "status": "AVAILABLE",
                    "message": "Ready",
                    "lastUsed": "2023-01-01T00:00:00Z",
                    "primary": true,
                    "workspaceName": "test-workspace",
                    "visibility": "PRIVATE",
                    "workDir": "/work",
                    "credentialsId": "cred-1",
                    "region": "us-east-1"
                }]
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url("test-token".to_string(), &mock_server.uri())
            .unwrap();

        let response = client.list_compute_envs(123, None).await.unwrap();
        assert_eq!(response.compute_envs.len(), 1);

        let ce = &response.compute_envs[0];
        assert_eq!(ce.id, "ce-1");
        assert_eq!(ce.name, "test-env");
        assert_eq!(ce.platform, "aws-batch");
        assert!(matches!(ce.status, ComputeEnvStatus::Available));
        assert_eq!(ce.message, Some("Ready".to_string()));
        assert!(ce.primary.unwrap_or(false));
        assert_eq!(ce.workspace_name, "test-workspace");
        assert_eq!(ce.visibility, "PRIVATE");
        assert_eq!(ce.work_dir, "/work");
        assert_eq!(ce.credentials_id, "cred-1");
        assert_eq!(ce.region, Some("us-east-1".into()));
    }

    #[tokio::test]
    async fn test_list_compute_envs_with_status() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/compute-envs"))
            .and(query_param("workspaceId", "123"))
            .and(query_param("status", "AVAILABLE"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "computeEnvs": []
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url("test-token".to_string(), &mock_server.uri())
            .unwrap();

        let response = client
            .list_compute_envs(123, Some(ComputeEnvStatus::Available))
            .await
            .unwrap();
        assert_eq!(response.compute_envs.len(), 0);
    }

    #[tokio::test]
    async fn test_validate_compute_env_name() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/compute-envs/validate"))
            .and(query_param("workspaceId", "123"))
            .and(query_param("name", "test-env"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url("test-token".to_string(), &mock_server.uri())
            .unwrap();

        let result = client.validate_compute_env_name(123, "test-env").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_compute_env_name_duplicate() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/compute-envs/validate"))
            .and(query_param("workspaceId", "123"))
            .and(query_param("name", "existing-env"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(
                ResponseTemplate::new(409)
                    .set_body_json(json!({"message": "Compute environment name already exists"})),
            )
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url("test-token".to_string(), &mock_server.uri())
            .unwrap();

        let result = client.validate_compute_env_name(123, "existing-env").await;
        assert!(result.is_err());
        if let Err(SeqeraError::Api { status, .. }) = result {
            assert_eq!(status, reqwest::StatusCode::CONFLICT);
        } else {
            panic!("Expected API error with conflict status");
        }
    }

    #[tokio::test]
    async fn test_update_compute_env() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/compute-envs/ce-1"))
            .and(query_param("workspaceId", "123"))
            .and(header("authorization", "Bearer test-token"))
            .and(body_json(json!({
                "name": "updated-env",
                "credentialsId": "cred-2"
            })))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url("test-token".to_string(), &mock_server.uri())
            .unwrap();

        let request = UpdateComputeEnvRequest {
            name: "updated-env".to_string(),
            credentials_id: "cred-2".to_string(),
        };

        let result = client.update_compute_env("ce-1", 123, request).await;
        assert!(result.is_ok());
    }
}
