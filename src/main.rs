use clap::{Parser, Subcommand, CommandFactory};
use env_logger;
use log::error;
use std::env;
use std::process;
use pform::SeqeraClient;
use pform::models::compute_env::ComputeEnvStatus;
use clap_complete;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn on verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Organization commands
    #[command(subcommand)]
    Orgs(OrgCommands),
    /// Workspace commands
    #[command(subcommand)]
    Workspaces(WorkspaceCommands),
    /// Compute environment commands
    #[command(subcommand)]
    ComputeEnv(ComputeEnvCommands),
    /// Generate shell completions
    GenerateCompletions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Launch interactive console
    Console,
}

#[derive(Debug, Subcommand)]
enum OrgCommands {
    /// List all organizations
    List,
    /// Get organization details
    Get {
        /// Organization ID
        #[arg(long)]
        id: i64,
    },
    /// Validate organization name
    ValidateName {
        /// Organization name to validate
        name: String,
    },
}

#[derive(Debug, Subcommand)]
enum WorkspaceCommands {
    /// List all workspaces in an organization
    List {
        /// Organization ID
        #[arg(long, conflicts_with = "org_name")]
        org_id: Option<i64>,
        /// Organization name
        #[arg(long, conflicts_with = "org_id")]
        org_name: Option<String>,
    },
    /// View workspace details
    View {
        /// Organization ID
        #[arg(long)]
        org_id: i64,
        /// Workspace ID
        #[arg(long)]
        id: i64,
    },
}

#[derive(Debug, Subcommand)]
pub enum ComputeEnvCommands {
    /// List compute environments
    List {
        /// Workspace ID
        #[arg(long)]
        workspace_id: i64,
        
        /// Filter by status (CREATING, AVAILABLE, ERRORED, INVALID)
        #[arg(long)]
        status: Option<String>,
    },
    
    /// Get compute environment details
    Get {
        /// Compute environment ID
        compute_env_id: String,
        
        /// Workspace ID
        #[arg(long)]
        workspace_id: i64,
    },
    
    /// Validate compute environment name
    ValidateName {
        /// Workspace ID
        #[arg(long)]
        workspace_id: i64,
        
        /// Name to validate
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.verbose {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    match cli.command {
        Commands::Console => {
            pform::tui::run().await?;
        }
        Commands::GenerateCompletions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "pform", &mut std::io::stdout());
        }
        Commands::Orgs(cmd) => {
            let token = env::var("TOWER_ACCESS_TOKEN")
                .map_err(|_| "TOWER_ACCESS_TOKEN environment variable not set")?;
            let client = SeqeraClient::new(token)?;
            match cmd {
                OrgCommands::List => {
                    let orgs = client.list_organizations().await?;
                    println!("{:#?}", orgs);
                }
                OrgCommands::Get { id } => {
                    let org = client.get_organization(id).await?;
                    println!("{:#?}", org);
                }
                OrgCommands::ValidateName { name } => {
                    let result = client.validate_organization_name(&name).await?;
                    println!("{:#?}", result);
                }
            }
        }
        Commands::Workspaces(cmd) => {
            let token = env::var("TOWER_ACCESS_TOKEN")
                .map_err(|_| "TOWER_ACCESS_TOKEN environment variable not set")?;
            let client = SeqeraClient::new(token)?;
            match cmd {
                WorkspaceCommands::List { org_id, org_name } => {
                    let workspaces = if let Some(id) = org_id {
                        client.list_workspaces(id).await?
                    } else if let Some(name) = org_name {
                        // First find the organization ID
                        let orgs = client.list_organizations().await?;
                        let org = orgs.organizations.into_iter()
                            .find(|o| o.name == name)
                            .ok_or_else(|| format!("Organization '{}' not found", name))?;
                        client.list_workspaces(org.id).await?
                    } else {
                        error!("Either org_id or org_name must be provided");
                        process::exit(1);
                    };
                    println!("{:#?}", workspaces);
                }
                WorkspaceCommands::View { org_id, id } => {
                    let workspace = client.get_workspace(org_id, id).await?;
                    println!("{:#?}", workspace);
                }
            }
        }
        Commands::ComputeEnv(cmd) => {
            let token = env::var("TOWER_ACCESS_TOKEN")
                .map_err(|_| "TOWER_ACCESS_TOKEN environment variable not set")?;
            let client = SeqeraClient::new(token)?;
            match cmd {
                ComputeEnvCommands::List { workspace_id, status } => {
                    let status = status.map(|s| match s.to_uppercase().as_str() {
                        "CREATING" => ComputeEnvStatus::Creating,
                        "AVAILABLE" => ComputeEnvStatus::Available,
                        "ERRORED" => ComputeEnvStatus::Errored,
                        "INVALID" => ComputeEnvStatus::Invalid,
                        _ => ComputeEnvStatus::Available,
                    });
                    let envs = client.list_compute_envs(workspace_id, status).await?;
                    println!("{:#?}", envs);
                }
                ComputeEnvCommands::Get { compute_env_id, workspace_id } => {
                    let env = client.get_compute_env(&compute_env_id, workspace_id).await?;
                    println!("{:#?}", env);
                }
                ComputeEnvCommands::ValidateName { workspace_id, name } => {
                    let result = client.validate_compute_env_name(workspace_id, &name).await?;
                    println!("{:#?}", result);
                }
            }
        }
    }

    Ok(())
}



