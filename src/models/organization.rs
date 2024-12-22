use serde::{Deserialize, Serialize};
use super::common::{Validate, ValidationError, validation, OrgId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "orgId")]
    pub id: OrgId,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListOrganizationsResponse {
    pub organizations: Vec<Organization>,
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationResponse {
    pub organization: Organization,
}

#[derive(Debug, Default)]
pub struct CreateOrganizationRequestBuilder {
    name: Option<String>,
    full_name: Option<String>,
    description: Option<String>,
    location: Option<String>,
    website: Option<String>,
    logo_id: Option<String>,
}

impl CreateOrganizationRequestBuilder {
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

    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn website(mut self, website: impl Into<String>) -> Self {
        self.website = Some(website.into());
        self
    }

    pub fn logo_id(mut self, logo_id: impl Into<String>) -> Self {
        self.logo_id = Some(logo_id.into());
        self
    }

    pub fn build(self) -> Result<CreateOrganizationRequest, &'static str> {
        let name = self.name.ok_or("name is required")?;
        let full_name = self.full_name.ok_or("full_name is required")?;

        Ok(CreateOrganizationRequest {
            organization: CreateOrganizationInner {
                name,
                full_name,
                description: self.description,
                location: self.location,
                website: self.website,
            },
            logo_id: self.logo_id,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrganizationInner {
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {
    pub organization: CreateOrganizationInner,
    #[serde(rename = "logoId")]
    pub logo_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrganizationRequest {
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "logoId")]
    pub logo_id: Option<String>,
    pub paying: bool,
}

impl Validate for CreateOrganizationRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        // Name: max 40 chars, modified pattern without look-ahead
        validation::validate_max_length(&self.organization.name, 40, "name")?;
        lazy_static::lazy_static! {
            static ref NAME_PATTERN: regex::Regex = regex::Regex::new(
                r"^[a-zA-Z\d][-_a-zA-Z\d]{1,38}[a-zA-Z\d]$"
            ).unwrap();
        }
        validation::validate_pattern(&self.organization.name, &NAME_PATTERN, "name")?;

        // Full name: max 100 chars
        validation::validate_max_length(&self.organization.full_name, 100, "fullName")?;

        // Optional fields
        if let Some(ref desc) = self.organization.description {
            validation::validate_max_length(desc, 1000, "description")?;
        }
        if let Some(ref location) = self.organization.location {
            validation::validate_max_length(location, 100, "location")?;
        }

        Ok(())
    }
}

impl Validate for UpdateOrganizationRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        // Name: max 40 chars, modified pattern without look-ahead
        validation::validate_max_length(&self.name, 40, "name")?;
        lazy_static::lazy_static! {
            static ref NAME_PATTERN: regex::Regex = regex::Regex::new(
                r"^[a-zA-Z\d][-_a-zA-Z\d]{1,38}[a-zA-Z\d]$"
            ).unwrap();
        }
        validation::validate_pattern(&self.name, &NAME_PATTERN, "name")?;

        // Full name: max 100 chars
        validation::validate_max_length(&self.full_name, 100, "fullName")?;

        // Optional fields
        if let Some(ref desc) = self.description {
            validation::validate_max_length(desc, 1000, "description")?;
        }
        if let Some(ref location) = self.location {
            validation::validate_max_length(location, 100, "location")?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationQuotasResponse {
    pub quotas: OrganizationQuotas,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationQuotas {
    #[serde(rename = "maxWorkspaces")]
    pub max_workspaces: i32,
    #[serde(rename = "maxMembers")]
    pub max_members: i32,
    #[serde(rename = "maxTeams")]
    pub max_teams: i32,
    #[serde(rename = "maxPipelinesPerWorkspace")]
    pub max_pipelines_per_workspace: i32,
    #[serde(rename = "maxParticipantsPerWorkspace")]
    pub max_participants_per_workspace: i32,
    #[serde(rename = "maxDatasetsPerWorkspace")]
    pub max_datasets_per_workspace: i32,
    #[serde(rename = "maxVersionsPerDataset")]
    pub max_versions_per_dataset: i32,
    #[serde(rename = "maxRuns")]
    pub max_runs: i32,
    #[serde(rename = "maxLabelsPerWorkspace")]
    pub max_labels_per_workspace: i32,
    #[serde(rename = "maxDataStudiosRunning")]
    pub max_data_studios_running: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_name_validation() {
        let request = CreateOrganizationRequest {
            organization: CreateOrganizationInner {
                name: "a".repeat(41),
                full_name: "Test Org".to_string(),
                description: None,
                location: None,
                website: None,
            },
            logo_id: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_valid_organization_request() {
        let request = CreateOrganizationRequest {
            organization: CreateOrganizationInner {
                name: "test-org".to_string(),
                full_name: "Test Organization".to_string(),
                description: Some("Description".to_string()),
                location: Some("Location".to_string()),
                website: None,
            },
            logo_id: None,
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_organization_builder() {
        let request = CreateOrganizationRequestBuilder::new()
            .name("test-org")
            .full_name("Test Organization")
            .description("A test organization")
            .location("Test Location")
            .website("https://test.org")
            .logo_id("logo123")
            .build()
            .unwrap();

        assert_eq!(request.organization.name, "test-org");
        assert_eq!(request.organization.full_name, "Test Organization");
        assert_eq!(request.organization.description.unwrap(), "A test organization");
        assert_eq!(request.organization.location.unwrap(), "Test Location");
        assert_eq!(request.organization.website.unwrap(), "https://test.org");
        assert_eq!(request.logo_id.unwrap(), "logo123");
    }
} 