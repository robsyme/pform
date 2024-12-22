use clap::{Parser, Subcommand, CommandFactory};
use env_logger;
use log::error;
use std::env;
use std::process;
use pform::SeqeraClient;
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
async fn main() {
    env_logger::init();

    let cli = Cli::parse();

    if let Commands::GenerateCompletions { shell } = cli.command {
        clap_complete::generate(
            shell,
            &mut Cli::command(),
            "pform",
            &mut std::io::stdout()
        );
        return;
    }

    let token = match env::var("TOWER_ACCESS_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            error!("TOWER_ACCESS_TOKEN environment variable not set");
            process::exit(1);
        }
    };

    let mut client = match SeqeraClient::new(token) {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create client: {}", e);
            process::exit(1);
        }
    };

    client.set_verbose(cli.verbose);

    match cli.command {
        Commands::Orgs(cmd) => match cmd {
            OrgCommands::List => match client.list_organizations().await {
                Ok(response) => {
                    println!("Found {} organizations:", response.organizations.len());
                    for org in response.organizations {
                        println!("{:<8} {}", org.id.0, org.name);
                        if let Some(desc) = org.description {
                            println!("  {}", desc);
                        }
                        println!();
                    }
                }
                Err(e) => {
                    error!("Failed to list organizations: {}", e);
                    process::exit(1);
                }
            },
            OrgCommands::Get { id } => match client.get_organization(id).await {
                Ok(org) => {
                    println!("ID:          {}", org.id.0);
                    println!("Name:        {}", org.name);
                    if let Some(desc) = org.description {
                        println!("Description: {}", desc);
                    }
                }
                Err(e) => {
                    error!("Failed to get organization: {}", e);
                    process::exit(1);
                }
            },
            OrgCommands::ValidateName { name } => match client.validate_organization_name(&name).await {
                Ok(_) => println!("Organization name '{}' is valid", name),
                Err(e) => {
                    error!("Organization name validation failed: {}", e);
                    process::exit(1);
                }
            },
        },
        Commands::Workspaces(cmd) => match cmd {
            WorkspaceCommands::List { org_id, org_name } => {
                let org_id = match (org_id, org_name) {
                    (Some(id), _) => id,
                    (None, Some(name)) => {
                        match client.find_organization_by_name(&name).await {
                            Ok(Some(org)) => org.id.0,
                            Ok(None) => {
                                error!("Organization '{}' not found", name);
                                process::exit(1);
                            }
                            Err(e) => {
                                error!("Failed to find organization: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                    (None, None) => {
                        error!("Either --org-id or --org-name must be specified");
                        process::exit(1);
                    }
                };

                match client.list_workspaces(org_id).await {
                    Ok(workspaces) => {
                        println!("Found {} workspaces:", workspaces.workspaces.len());
                        for workspace in workspaces.workspaces {
                            println!("{} ({})", workspace.name, workspace.id);
                            if let Some(desc) = workspace.description {
                                println!("  {}", desc);
                            }
                            println!("  Visibility: {}", workspace.visibility);
                            println!();
                        }
                    }
                    Err(e) => {
                        error!("Failed to list workspaces: {}", e);
                        process::exit(1);
                    }
                }
            },
            WorkspaceCommands::View { org_id, id } => {
                match client.get_workspace(org_id, id).await {
                    Ok(workspace) => {
                        println!("ID:          {}", workspace.id);
                        println!("Name:        {}", workspace.name);
                        println!("Full Name:   {}", workspace.full_name);
                        if let Some(desc) = workspace.description {
                            println!("Description: {}", desc);
                        }
                        println!("Visibility:  {}", workspace.visibility);
                    }
                    Err(e) => {
                        error!("Failed to get workspace: {}", e);
                        process::exit(1);
                    }
                }
            }
        },
        Commands::ComputeEnv(cmd) => match cmd {
            ComputeEnvCommands::List { workspace_id, status: _ } => {
                match client.list_compute_envs(workspace_id, None).await {
                    Ok(response) => {
                        println!("Found {} compute environments:", response.compute_envs.len());
                        for ce in response.compute_envs {
                            println!("\nID:          {}", ce.id);
                            println!("Name:        {}", ce.name);
                            println!("Platform:    {}", ce.platform);
                            println!("Status:      {}", ce.status);
                            if let Some(message) = &ce.message {
                                println!("Message:     {}", message);   
                            }
                            if let Some(last_used) = &ce.last_used {
                                println!("Last Used:   {}", last_used);
                            }
                            if let Some(primary) = ce.primary {
                                println!("Primary:     {}", primary);
                            }
                            println!("Workspace:   {}", ce.workspace_name);
                            println!("Visibility:  {}", ce.visibility);
                            println!("Work Dir:    {}", ce.work_dir);
                            if let Some(region) = &ce.region {
                                println!("Region:      {}", region);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to list compute environments: {}", e);
                        process::exit(1);
                    }
                }
            },
            ComputeEnvCommands::Get { compute_env_id, workspace_id } => {
                match client.get_compute_env(&compute_env_id, workspace_id).await {
                    Ok(ce) => {
                        println!("ID:            {}", ce.id);
                        println!("Name:          {}", ce.name);
                        if let Some(desc) = &ce.description {
                            println!("Description:   {}", desc);
                        }
                        println!("Platform:      {:?}", ce.platform);
                        println!("Status:        {}", ce.status);
                        if let Some(message) = ce.message {
                            println!("Message:       {}", message);
                        }
                        println!("Date Created:  {}", ce.date_created);
                        println!("Last Updated:  {}", ce.last_updated);
                        println!("Last Used:     {}", ce.last_used);
                        if let Some(primary) = ce.primary {
                            println!("Primary:       {}", primary);
                        }
                        println!("Org ID:        {}", ce.org_id);
                        println!("Workspace ID:  {}", ce.workspace_id);
                        
                    }
                    Err(e) => {
                        error!("Failed to get compute environment: {}", e);
                        process::exit(1);
                    }
                }
            },
            ComputeEnvCommands::ValidateName { workspace_id, name } => {
                match client.validate_compute_env_name(workspace_id, &name).await {
                    Ok(_) => println!("Compute environment name '{}' is valid", name),
                    Err(e) => {
                        error!("Name validation failed: {}", e);
                        process::exit(1);
                    }
                }
            },
        },
        Commands::GenerateCompletions { .. } => (),
    }
}



