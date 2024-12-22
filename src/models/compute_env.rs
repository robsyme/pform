use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fmt;
// Platform-specific configurations will be in separate files
mod config;
pub use config::*;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ComputeEnvStatus {
    Creating,
    Available,
    Errored,
    Invalid,
}

impl fmt::Display for ComputeEnvStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputeEnvStatus::Creating => write!(f, "CREATING"),
            ComputeEnvStatus::Available => write!(f, "AVAILABLE"),
            ComputeEnvStatus::Errored => write!(f, "ERRORED"),
            ComputeEnvStatus::Invalid => write!(f, "INVALID"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    AwsBatch,
    GoogleLifesciences,
    GoogleBatch,
    AzureBatch,
    K8sPlatform,
    EksPlatform,
    GkePlatform,
    UgePlatform,
    SlurmPlatform,
    LsfPlatform,
    AltairPlatform,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
    pub head: bool,
    pub compute: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub value: String,
    pub resource: bool,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "discriminator")]
pub enum ComputeConfig {
    #[serde(rename = "aws-batch")]
    AwsBatch(AwsBatchConfig),
    #[serde(rename = "google-lifesciences")]
    GoogleLifeSciences(GoogleLifeSciencesConfig),
    #[serde(rename = "google-batch")]
    GoogleBatch(GoogleBatchConfig),
    #[serde(rename = "azure-batch")]
    AzureBatch(AzureBatchConfig),
    #[serde(rename = "lsf-platform")]
    Lsf(LsfConfig),
    #[serde(rename = "slurm-platform")]
    Slurm(SlurmConfig),
    #[serde(rename = "k8s-platform")]
    Kubernetes(KubernetesConfig),
    #[serde(rename = "eks-platform")]
    Eks(EksConfig),
    #[serde(rename = "gke-platform")]
    Gke(GkeConfig),
    #[serde(rename = "uge-platform")]
    Uge(UgeConfig),
    #[serde(rename = "altair-platform")]
    Altair(AltairConfig),
    #[serde(rename = "moab-platform")]
    Moab(MoabConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeEnvDetails {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub platform: Platform,
    pub config: ComputeConfig,
    #[serde(rename = "dateCreated")]
    pub date_created: DateTime<Utc>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "lastUsed")]
    pub last_used: DateTime<Utc>,
    pub deleted: Option<bool>,
    pub status: ComputeEnvStatus,
    pub message: Option<String>,
    pub primary: Option<bool>,
    #[serde(rename = "credentialsId")]
    pub credentials_id: String,
    #[serde(rename = "managedIdentityId")]
    pub managed_identity_id: Option<String>,
    #[serde(rename = "orgId")]
    pub org_id: i64,
    #[serde(rename = "workspaceId")]
    pub workspace_id: i64,
    // pub labels: Option<Vec<Label>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetComputeEnvResponse {
    #[serde(rename = "computeEnv")]
    pub compute_env: ComputeEnvDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeEnv {
    #[serde(rename = "credentialsId")] 
    pub credentials_id: String,
    pub id: String,
    pub name: String,
    pub platform: String,
    pub region: Option<String>,
    pub status: ComputeEnvStatus,
    pub visibility: String,
    #[serde(rename = "workDir")]
    pub work_dir: String,
    #[serde(rename = "workspaceName")]
    pub workspace_name: String,
    pub message: Option<String>,
    #[serde(rename = "lastUsed")]
    pub last_used: Option<DateTime<Utc>>,
    pub primary: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListComputeEnvsResponse {
    #[serde(rename = "computeEnvs")]
    pub compute_envs: Vec<ComputeEnv>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateComputeEnvRequest {
    pub name: String,
    #[serde(rename = "credentialsId")]
    pub credentials_id: String,
}

