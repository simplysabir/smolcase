use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod commands;
mod config;
mod credential_manager;
mod crypto;
mod git;
mod types;
mod ui;

use commands::*;

#[derive(Parser)]
#[command(name = "smolcase")]
#[command(about = "Zero-infrastructure secret management for development teams.")]
#[command(version = "1.3.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new smolcase project
    Init {
        /// Project name
        #[arg(short, long)]
        name: Option<String>,
        /// Initialize with Git repository
        #[arg(short, long)]
        git: bool,
        /// Skip interactive prompts
        #[arg(long)]
        non_interactive: bool,
    },
    /// Interactive guided tutorial for new users
    Tutorial,
    /// Set up local credentials (run once to cache passwords)
    Configure,
    /// Clear cached credentials
    Logout,
    /// Add a new secret or file
    Add {
        /// Secret key or file path
        key: String,
        /// Secret value (will prompt if not provided)
        value: Option<String>,
        /// Users to share with (comma-separated)
        #[arg(short, long)]
        users: Option<String>,
        /// Groups to share with (comma-separated)
        #[arg(short, long)]
        groups: Option<String>,
    },
    /// Remove a secret
    Remove {
        /// Secret key to remove
        key: String,
    },
    /// List accessible secrets
    List,
    /// Get a secret value
    Get {
        /// Secret key
        key: String,
    },
    /// Run command with secrets as environment variables
    Run {
        /// Environment to use (optional)
        #[arg(short, long)]
        env: Option<String>,
        /// Command to execute
        #[arg(last = true)]
        command: Vec<String>,
    },
    /// Apply template with secret substitution
    Apply {
        /// Template file path
        template: PathBuf,
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Environment to use (optional)
        #[arg(short, long)]
        env: Option<String>,
    },
    /// Set up user access for a repository
    Setup {
        /// Repository URL or path
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Manage users (admin only)
    User {
        #[command(subcommand)]
        action: UserAction,
    },
    /// Manage groups (admin only)
    Group {
        #[command(subcommand)]
        action: GroupAction,
    },
    /// Export secrets as environment variables
    Export {
        /// Output format (env, json, yaml)
        #[arg(short, long, default_value = "env")]
        format: String,
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Environment to use (optional)
        #[arg(short, long)]
        env: Option<String>,
    },
    /// Import secrets from a file
    Import {
        /// Input file path
        file: PathBuf,
        /// Input format (env, json, yaml)
        #[arg(short, long, default_value = "env")]
        format: String,
    },
    /// Sync with Git repository
    Sync,
    /// Show project status
    Status,
}

#[derive(Subcommand)]
enum UserAction {
    /// Add a new user
    Add {
        username: String,
        #[arg(short, long)]
        email: Option<String>,
    },
    /// Remove a user
    Remove { username: String },
    /// List all users
    List,
    /// Reset user password
    Reset { username: String },
}

#[derive(Subcommand)]
enum GroupAction {
    /// Create a new group
    Create {
        name: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Delete a group
    Delete { name: String },
    /// List all groups
    List,
    /// Add users to a group
    AddUser { group: String, users: Vec<String> },
    /// Remove users from a group
    RemoveUser { group: String, users: Vec<String> },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            name,
            git,
            non_interactive,
        } => init::execute(name, git, non_interactive).await,
        Commands::Configure => configure::execute().await,
        Commands::Tutorial => tutorial::execute().await,
        Commands::Logout => logout::execute().await,
        Commands::Add {
            key,
            value,
            users,
            groups,
        } => add::execute(key, value, users, groups).await,
        Commands::Remove { key } => remove::execute(key).await,
        Commands::List => list::execute().await,
        Commands::Get { key } => get::execute(key).await,
        Commands::Run { env, command } => run::execute(env, command).await,
        Commands::Apply {
            template,
            output,
            env,
        } => apply::execute(template, output, env).await,
        Commands::Setup { repo } => setup::execute(repo).await,
        Commands::User { action } => user::execute(action).await,
        Commands::Group { action } => group::execute(action).await,
        Commands::Export {
            format,
            output,
            env,
        } => export::execute(format, output, env).await,
        Commands::Import { file, format } => import::execute(file, format).await,
        Commands::Sync => sync::execute().await,
        Commands::Status => status::execute().await,
    }
}
