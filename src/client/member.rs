use crate::errors::SeqeraError;
use crate::models::member::*;

impl super::SeqeraClient {
    pub async fn list_organization_members(&self, org_id: i64) -> Result<MemberList, SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/members", org_id))?;
        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }

    pub async fn list_organization_collaborators(&self, org_id: i64) -> Result<MemberList, SeqeraError> {
        let url = self.base_url.join(&format!("orgs/{}/collaborators", org_id))?;
        let request = self.client
            .get(url)
            .header("Authorization", self.auth_header())
            .build()?;

        let response = self.handle_response(request).await?;
        Ok(response.json().await?)
    }
} 