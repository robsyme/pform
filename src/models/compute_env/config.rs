use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::EnvVar;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseConfig {
    #[serde(rename = "workDir")]
    pub work_dir: String,
    #[serde(rename = "preRunScript")]
    pub pre_run_script: Option<String>,
    #[serde(rename = "postRunScript")]
    pub post_run_script: Option<String>,
    pub environment: Vec<EnvVar>,
    #[serde(rename = "nextflowConfig")]
    pub nextflow_config: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AwsForgeConfig {
    #[serde(rename = "type")]
    pub instance_type: String,
    #[serde(rename = "minCpus")]
    pub min_cpus: i32,
    #[serde(rename = "maxCpus")]
    pub max_cpus: i32,
    #[serde(rename = "gpuEnabled")]
    pub gpu_enabled: bool,
    #[serde(rename = "ebsAutoScale")]
    pub ebs_auto_scale: Option<bool>,
    #[serde(rename = "instanceTypes")]
    pub instance_types: Vec<String>,
    #[serde(rename = "allocStrategy")]
    pub alloc_strategy: Option<String>,
    #[serde(rename = "imageId")]
    pub image_id: Option<String>,
    #[serde(rename = "vpcId")]
    pub vpc_id: Option<String>,
    pub subnets: Vec<String>,
    #[serde(rename = "securityGroups")]
    pub security_groups: Vec<String>,
    #[serde(rename = "fsxMount")]
    pub fsx_mount: Option<String>,
    #[serde(rename = "fsxName")]
    pub fsx_name: Option<String>,
    #[serde(rename = "fsxSize")]
    pub fsx_size: Option<i32>,
    #[serde(rename = "disposeOnDeletion")]
    pub dispose_on_deletion: Option<bool>,
    #[serde(rename = "ec2KeyPair")]
    pub ec2_key_pair: Option<String>,
    #[serde(rename = "allowBuckets")]
    pub allow_buckets: Vec<String>,
    #[serde(rename = "ebsBlockSize")]
    pub ebs_block_size: Option<i32>,
    #[serde(rename = "fusionEnabled")]
    pub fusion_enabled: Option<bool>,
    #[serde(rename = "bidPercentage")]
    pub bid_percentage: Option<i32>,
    #[serde(rename = "efsCreate")]
    pub efs_create: bool,
    #[serde(rename = "efsId")]
    pub efs_id: Option<String>,
    #[serde(rename = "efsMount")]
    pub efs_mount: Option<String>,
    #[serde(rename = "dragenEnabled")]
    pub dragen_enabled: bool,
    #[serde(rename = "dragenAmiId")]
    pub dragen_ami_id: String,
    #[serde(rename = "ebsBootSize")]
    pub ebs_boot_size: Option<i32>,
    #[serde(rename = "ecsConfig")]
    pub ecs_config: Option<String>,
    #[serde(rename = "fargateHeadEnabled")]
    pub fargate_head_enabled: Option<bool>,
    #[serde(rename = "arm64Enabled")]
    pub arm64_enabled: Option<bool>,
    #[serde(rename = "dragenInstanceType")]
    pub dragen_instance_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AwsBatchConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    pub volumes: Vec<String>,
    pub region: String,
    #[serde(rename = "computeQueue")]
    pub compute_queue: Option<String>,
    #[serde(rename = "dragenQueue")]
    pub dragen_queue: Option<String>,
    #[serde(rename = "dragenInstanceType")]
    pub dragen_instance_type: String,
    #[serde(rename = "computeJobRole")]
    pub compute_job_role: Option<String>,
    #[serde(rename = "executionRole")]
    pub execution_role: String,
    #[serde(rename = "headQueue")]
    pub head_queue: String,
    #[serde(rename = "headJobRole")]
    pub head_job_role: Option<String>,
    #[serde(rename = "cliPath")]
    pub cli_path: Option<String>,
    #[serde(rename = "headJobCpus")]
    pub head_job_cpus: Option<i32>,
    #[serde(rename = "headJobMemoryMb")]
    pub head_job_memory_mb: Option<i32>,
    #[serde(rename = "waveEnabled")]
    pub wave_enabled: bool,
    #[serde(rename = "fusion2Enabled")]
    pub fusion2_enabled: bool,
    #[serde(rename = "nvnmeStorageEnabled")]
    pub nvnme_storage_enabled: bool,
    #[serde(rename = "logGroup")]
    pub log_group: Option<String>,
    pub forge: AwsForgeConfig,
    #[serde(rename = "forgedResources")]
    pub forged_resources: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleLifeSciencesConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    pub region: String,
    pub zones: Vec<String>,
    pub location: String,
    pub preemptible: bool,
    #[serde(rename = "bootDiskSizeGb")]
    pub boot_disk_size_gb: i32,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "sshDaemon")]
    pub ssh_daemon: bool,
    #[serde(rename = "sshImage")]
    pub ssh_image: String,
    #[serde(rename = "debugMode")]
    pub debug_mode: i32,
    #[serde(rename = "copyImage")]
    pub copy_image: String,
    #[serde(rename = "usePrivateAddress")]
    pub use_private_address: bool,
    pub labels: HashMap<String, String>,
    #[serde(rename = "headJobCpus")]
    pub head_job_cpus: i32,
    #[serde(rename = "headJobMemoryMb")]
    pub head_job_memory_mb: i32,
    #[serde(rename = "nfsTarget")]
    pub nfs_target: String,
    #[serde(rename = "nfsMount")]
    pub nfs_mount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleBatchConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    pub location: String,
    pub spot: bool,
    #[serde(rename = "bootDiskSizeGb")]
    pub boot_disk_size_gb: i32,
    #[serde(rename = "cpuPlatform")]
    pub cpu_platform: String,
    #[serde(rename = "machineType")]
    pub machine_type: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "sshDaemon")]
    pub ssh_daemon: bool,
    #[serde(rename = "sshImage")]
    pub ssh_image: String,
    #[serde(rename = "debugMode")]
    pub debug_mode: i32,
    #[serde(rename = "copyImage")]
    pub copy_image: String,
    #[serde(rename = "usePrivateAddress")]
    pub use_private_address: bool,
    pub labels: HashMap<String, String>,
    #[serde(rename = "headJobCpus")]
    pub head_job_cpus: i32,
    #[serde(rename = "headJobMemoryMb")]
    pub head_job_memory_mb: i32,
    #[serde(rename = "nfsTarget")]
    pub nfs_target: String,
    #[serde(rename = "nfsMount")]
    pub nfs_mount: String,
    #[serde(rename = "waveEnabled")]
    pub wave_enabled: bool,
    #[serde(rename = "fusion2Enabled")]
    pub fusion2_enabled: bool,
    #[serde(rename = "serviceAccount")]
    pub service_account: String,
    pub network: String,
    pub subnetwork: String,
    #[serde(rename = "headJobInstanceTemplate")]
    pub head_job_instance_template: String,
    #[serde(rename = "computeJobsInstanceTemplate")]
    pub compute_jobs_instance_template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureForgeConfig {
    #[serde(rename = "vmType")]
    pub vm_type: String,
    #[serde(rename = "vmCount")]
    pub vm_count: i32,
    #[serde(rename = "autoScale")]
    pub auto_scale: bool,
    #[serde(rename = "disposeOnDeletion")]
    pub dispose_on_deletion: Option<bool>,
    #[serde(rename = "containerRegIds")]
    pub container_reg_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteJobsOnCompletion {
    OnSuccess,
    Always,
    Never,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureBatchConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    pub region: String,
    #[serde(rename = "headPool")]
    pub head_pool: Option<String>,
    #[serde(rename = "autoPoolMode")]
    pub auto_pool_mode: Option<bool>,
    pub forge: AzureForgeConfig,
    #[serde(rename = "tokenDuration")]
    pub token_duration: Option<String>,
    #[serde(rename = "deleteJobsOnCompletion")]
    pub delete_jobs_on_completion: DeleteJobsOnCompletion,
    #[serde(rename = "deletePoolsOnCompletion")]
    pub delete_pools_on_completion: bool,
    #[serde(rename = "waveEnabled")]
    pub wave_enabled: bool,
    #[serde(rename = "fusion2Enabled")]
    pub fusion2_enabled: bool,
    #[serde(rename = "managedIdentityClientId")]
    pub managed_identity_client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseGridConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    #[serde(rename = "launchDir")]
    pub launch_dir: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "hostName")]
    pub host_name: String,
    pub port: i32,
    #[serde(rename = "headQueue")]
    pub head_queue: String,
    #[serde(rename = "computeQueue")]
    pub compute_queue: String,
    #[serde(rename = "maxQueueSize")]
    pub max_queue_size: i32,
    #[serde(rename = "headJobOptions")]
    pub head_job_options: String,
    #[serde(rename = "propagateHeadJobOptions")]
    pub propagate_head_job_options: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LsfConfig {
    #[serde(flatten)]
    pub base: BaseGridConfig,
    #[serde(rename = "unitForLimits")]
    pub unit_for_limits: String,
    #[serde(rename = "perJobMemLimit")]
    pub per_job_mem_limit: bool,
    #[serde(rename = "perTaskReserve")]
    pub per_task_reserve: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlurmConfig {
    #[serde(flatten)]
    pub base: BaseGridConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PodCleanup {
    OnSuccess,
    Always,
    Never,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseKubernetesConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    pub server: String,
    #[serde(rename = "sslCert")]
    pub ssl_cert: String,
    pub namespace: String,
    #[serde(rename = "computeServiceAccount")]
    pub compute_service_account: String,
    #[serde(rename = "headServiceAccount")]
    pub head_service_account: String,
    #[serde(rename = "storageClaimName")]
    pub storage_claim_name: String,
    #[serde(rename = "storageMountPath")]
    pub storage_mount_path: String,
    #[serde(rename = "podCleanup")]
    pub pod_cleanup: PodCleanup,
    #[serde(rename = "headPodSpec")]
    pub head_pod_spec: String,
    #[serde(rename = "servicePodSpec")]
    pub service_pod_spec: String,
    #[serde(rename = "headJobCpus")]
    pub head_job_cpus: i32,
    #[serde(rename = "headJobMemoryMb")]
    pub head_job_memory_mb: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KubernetesConfig {
    #[serde(flatten)]
    pub base: BaseKubernetesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EksConfig {
    #[serde(flatten)]
    pub base: BaseKubernetesConfig,
    pub region: String,
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    #[serde(rename = "waveEnabled")]
    pub wave_enabled: bool,
    #[serde(rename = "fusion2Enabled")]
    pub fusion2_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GkeConfig {
    #[serde(flatten)]
    pub base: BaseKubernetesConfig,
    pub region: String,
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    #[serde(rename = "fusion2Enabled")]
    pub fusion2_enabled: bool,
    #[serde(rename = "waveEnabled")]
    pub wave_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UgeConfig {
    #[serde(flatten)]
    pub base: BaseGridConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AltairConfig {
    #[serde(flatten)]
    pub base: BaseGridConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoabConfig {
    #[serde(flatten)]
    pub base: BaseGridConfig,
} 