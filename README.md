# pform

A Rust client for the Seqera Platform API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pform = "0.1.0"
```

## Library Usage

```rust
use pform::SeqeraClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client with your API token (uses default base URL)
    let client = SeqeraClient::new("your-api-token".to_string())?;

    // Or specify a custom base URL
    let client = SeqeraClient::with_base_url(
        "your-api-token".to_string(),
        "https://custom.seqera.instance/"
    )?;

    // List organizations
    let orgs = client.list_organizations().await?;
    println!("Found {} organizations", orgs.organizations.len());

    // Get organization details
    let org = client.get_organization(123).await?;
    println!("Organization name: {}", org.name);

    // List workspaces in an organization
    let workspaces = client.list_workspaces(123).await?;
    println!("Found {} workspaces", workspaces.workspaces.len());

    Ok(())
}
```

## Configuration

### Base URL

The client uses `https://api.cloud.seqera.io/` as the default base URL. You can override this by:

1. Using the `with_base_url` constructor in library mode:
```rust
let client = SeqeraClient::with_base_url(token, "https://custom.seqera.instance/")?;
```

2. Setting the `TOWER_API_URL` environment variable in CLI mode

## CLI Usage

The package includes a command-line interface for interacting with the Seqera Platform.

### Authentication

Set your Seqera Platform API token in the environment:

```bash
export TOWER_ACCESS_TOKEN="your-api-token"
```

### Commands

#### Organization Commands
```bash
# List all organizations
pform orgs list

# Get organization details
pform orgs get --id 123

# Validate organization name
pform orgs validate-name <name>
```

#### Workspace Commands
```bash
# List workspaces in an organization (using org ID)
pform workspaces list --org-id 123

# List workspaces in an organization (using org name)
pform workspaces list --org-name "my-org"

# View workspace details
pform workspaces view --org-id 123 --id 456
```

#### Compute Environment Commands
```bash
# List compute environments
pform compute-env list --workspace-id 123

# List compute environments with status filter
pform compute-env list --workspace-id 123 --status AVAILABLE

# Get compute environment details
pform compute-env get --workspace-id 123 <compute-env-id>

# Validate compute environment name
pform compute-env validate-name --workspace-id 123 <name>
```

#### Shell Completion

Generate shell completions for your preferred shell:

```bash
# Generate bash completions
pform generate-completions bash > ~/.local/share/bash-completion/completions/pform

# Generate zsh completions
pform generate-completions zsh > ~/.zfunc/_pform

# Generate fish completions
pform generate-completions fish > ~/.config/fish/completions/pform.fish
```

For zsh, make sure `~/.zfunc` is in your `fpath` by adding this to your `.zshrc`:
```zsh
fpath=(~/.zfunc $fpath)
autoload -U compinit; compinit
```

### Verbose Mode

Add the `--verbose` flag to any command to see detailed HTTP request/response information:
```bash
pform --verbose orgs list
```

## Features

Currently implemented endpoints:
- Organizations
  - List organizations
  - Get organization details
  - Validate organization name
- Workspaces
  - List workspaces in an organization (by ID or name)
  - View workspace details
- Compute Environments
  - List compute environments
  - Get compute environment details
  - Validate compute environment name
  - Update compute environment

## Authentication

You'll need an API token from the Seqera Platform. You can generate one in your account settings at https://cloud.seqera.io.

## License

This project is licensed under the MIT License. 