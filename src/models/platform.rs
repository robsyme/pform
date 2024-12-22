use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pub id: String,
    pub name: String,
    #[serde(rename = "credentialsProviders")]
    pub credentials_providers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPlatformsResponse {
    pub platforms: Vec<Platform>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobQueue {
    pub name: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSystem {
    pub id: String,
    pub dns: String,
    pub mount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EfsFileSystem {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vpc {
    pub id: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroup {
    pub id: String,
    pub name: String,
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subnet {
    pub id: String,
    pub zone: String,
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filestore {
    pub target: String,
    pub name: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlatformMetaInfo {
    Type1 {
        warnings: Vec<String>,
        #[serde(rename = "jobQueues")]
        job_queues: Vec<JobQueue>,
        buckets: Vec<Bucket>,
        #[serde(rename = "fileSystems")]
        file_systems: Vec<FileSystem>,
        #[serde(rename = "efsFileSystems")]
        efs_file_systems: Vec<EfsFileSystem>,
        #[serde(rename = "keyPairs")]
        key_pairs: Vec<String>,
        vpcs: Vec<Vpc>,
        images: Vec<Image>,
        #[serde(rename = "securityGroups")]
        security_groups: Vec<SecurityGroup>,
        subnets: Vec<Subnet>,
        #[serde(rename = "instanceFamilies")]
        instance_families: Vec<String>,
        #[serde(rename = "allocStrategy")]
        alloc_strategy: Vec<String>,
    },
    Type2 {
        locations: Vec<String>,
        warnings: Vec<String>,
        zones: Vec<String>,
        buckets: Vec<Bucket>,
        filestores: Vec<Filestore>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformDetailsResponse {
    pub metainfo: PlatformMetaInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformRegion {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPlatformRegionsResponse {
    pub regions: Vec<PlatformRegion>,
} 