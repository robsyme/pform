use crate::errors::SeqeraError;
use crate::models::platform::{ListPlatformsResponse, PlatformDetailsResponse, ListPlatformRegionsResponse};
use crate::models::common::{OrgId, WorkspaceId};

impl super::SeqeraClient {
    pub async fn list_platforms(
        &self,
        workspace_id: Option<impl Into<WorkspaceId>>,
        org_id: Option<impl Into<OrgId>>,
    ) -> Result<ListPlatformsResponse, SeqeraError> {
        let mut url = self.base_url.join("platforms")?;

        if let Some(workspace_id) = workspace_id {
            url.query_pairs_mut()
                .append_pair("workspaceId", &workspace_id.into().0.to_string());
        }

        if let Some(org_id) = org_id {
            url.query_pairs_mut()
                .append_pair("orgId", &org_id.into().0.to_string());
        }

        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn get_platform_details(
        &self,
        platform_id: impl AsRef<str>,
        workspace_id: impl Into<WorkspaceId>,
        region_id: Option<&str>,
        credentials_id: Option<&str>,
    ) -> Result<PlatformDetailsResponse, SeqeraError> {
        let mut url = self.base_url.join(&format!("platforms/{}", platform_id.as_ref()))?;
        let mut query_pairs = url.query_pairs_mut();
        
        query_pairs.append_pair("workspaceId", &workspace_id.into().0.to_string());
        
        if let Some(region_id) = region_id {
            query_pairs.append_pair("regionId", region_id);
        }
        
        if let Some(credentials_id) = credentials_id {
            query_pairs.append_pair("credentialsId", credentials_id);
        }
        
        drop(query_pairs);

        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn list_platform_regions(
        &self,
        platform_id: impl AsRef<str>,
        workspace_id: impl Into<WorkspaceId>,
    ) -> Result<ListPlatformRegionsResponse, SeqeraError> {
        let mut url = self.base_url.join(&format!("platforms/{}/regions", platform_id.as_ref()))?;
        url.query_pairs_mut()
            .append_pair("workspaceId", &workspace_id.into().0.to_string());

        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header, query_param};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_platform_details_type1() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/platforms/aws-batch"))
            .and(query_param("workspaceId", "123"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "metainfo": {
                    "warnings": ["Warning 1"],
                    "jobQueues": [{
                        "name": "queue1",
                        "state": "ENABLED"
                    }],
                    "buckets": [{
                        "path": "s3://bucket1"
                    }],
                    "fileSystems": [{
                        "id": "fs-1",
                        "dns": "fs1.example.com",
                        "mount": "/mnt/fs1"
                    }],
                    "efsFileSystems": [{
                        "id": "efs-1"
                    }],
                    "keyPairs": ["key1"],
                    "vpcs": [{
                        "id": "vpc-1",
                        "isDefault": true
                    }],
                    "images": [{
                        "id": "ami-1",
                        "name": "image1",
                        "description": "Test image"
                    }],
                    "securityGroups": [{
                        "id": "sg-1",
                        "name": "group1",
                        "vpcId": "vpc-1"
                    }],
                    "subnets": [{
                        "id": "subnet-1",
                        "zone": "us-east-1a",
                        "vpcId": "vpc-1"
                    }],
                    "instanceFamilies": ["t2", "t3"],
                    "allocStrategy": ["BEST_FIT"]
                }
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let response = client.get_platform_details("aws-batch", 123, None, None).await.unwrap();
        
        match response.metainfo {
            crate::models::platform::PlatformMetaInfo::Type1 { 
                warnings,
                job_queues,
                buckets,
                file_systems,
                efs_file_systems,
                key_pairs,
                vpcs,
                images,
                security_groups,
                subnets,
                instance_families,
                alloc_strategy,
            } => {
                assert_eq!(warnings, vec!["Warning 1"]);
                assert_eq!(job_queues[0].name, "queue1");
                assert_eq!(buckets[0].path, "s3://bucket1");
                assert_eq!(file_systems[0].id, "fs-1");
                assert_eq!(efs_file_systems[0].id, "efs-1");
                assert_eq!(key_pairs, vec!["key1"]);
                assert_eq!(vpcs[0].id, "vpc-1");
                assert_eq!(images[0].id, "ami-1");
                assert_eq!(security_groups[0].id, "sg-1");
                assert_eq!(subnets[0].id, "subnet-1");
                assert_eq!(instance_families, vec!["t2", "t3"]);
                assert_eq!(alloc_strategy, vec!["BEST_FIT"]);
            },
            _ => panic!("Expected Type1 response"),
        }
    }

    #[tokio::test]
    async fn test_get_platform_details_type2() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/platforms/google-cloud"))
            .and(query_param("workspaceId", "123"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "metainfo": {
                    "locations": ["us-east1"],
                    "warnings": ["Warning 1"],
                    "zones": ["us-east1-b"],
                    "buckets": [{
                        "path": "gs://bucket1"
                    }],
                    "filestores": [{
                        "target": "target1",
                        "name": "filestore1",
                        "location": "us-east1"
                    }]
                }
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let response = client.get_platform_details("google-cloud", 123, None, None).await.unwrap();
        
        match response.metainfo {
            crate::models::platform::PlatformMetaInfo::Type2 { 
                locations,
                warnings,
                zones,
                buckets,
                filestores,
            } => {
                assert_eq!(locations, vec!["us-east1"]);
                assert_eq!(warnings, vec!["Warning 1"]);
                assert_eq!(zones, vec!["us-east1-b"]);
                assert_eq!(buckets[0].path, "gs://bucket1");
                assert_eq!(filestores[0].target, "target1");
            },
            _ => panic!("Expected Type2 response"),
        }
    }

    #[tokio::test]
    async fn test_list_platform_regions() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/platforms/aws-batch/regions"))
            .and(query_param("workspaceId", "123"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "regions": [{
                    "id": "us-east-1",
                    "name": "US East (N. Virginia)"
                }, {
                    "id": "us-west-2",
                    "name": "US West (Oregon)"
                }]
            })))
            .mount(&mock_server)
            .await;

        let client = super::super::SeqeraClient::with_base_url(
            "test-token".to_string(),
            &mock_server.uri(),
        ).unwrap();

        let response = client.list_platform_regions("aws-batch", 123).await.unwrap();
        assert_eq!(response.regions.len(), 2);
        
        let region = &response.regions[0];
        assert_eq!(region.id, "us-east-1");
        assert_eq!(region.name, "US East (N. Virginia)");
        
        let region = &response.regions[1];
        assert_eq!(region.id, "us-west-2");
        assert_eq!(region.name, "US West (Oregon)");
    }
} 