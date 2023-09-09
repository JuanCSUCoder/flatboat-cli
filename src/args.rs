use clap::{Parser, Subcommand, Args};

#[derive(Subcommand)]
pub enum WorkspaceSubcommands {
    /// Creates a new workspace in the specified location
    Create {
        ws_name: String,
    }
}

#[derive(Subcommand)]
pub enum BotSubcommands {
    Create,
    BringUp,
    BringDown,
    Refresh,
}

#[derive(Subcommand)]
pub enum WorkloadSubcommands {
    Create,
    Deploy,
}

/// Workspace Subcommands
#[derive(Args)]
pub struct WorkspaceArgs {
    /// Workspace Sub-commands
    #[command(subcommand)]
    pub subcommand: WorkspaceSubcommands,
}

/// Bot Subcommands
#[derive(Args)]
pub struct BotArgs {
    /// Bot Sub-commands
    #[command(subcommand)]
    pub subcommand: BotSubcommands,
}

/// Workload Subcommands
#[derive(Args)]
pub struct WorkloadArgs {
    /// Workload Sub-commands
    #[command(subcommand)]
    subcommand: WorkloadSubcommands,
}

/// Commands
#[derive(Subcommand)]
pub enum Commands {
    /// Commands to create and manipulate a Dockerized ROS2 Workspace
    Workspace(WorkspaceArgs),

    /// Commands to create and manipulate a Dockerized Robotic Kubernetes Node
    Bot(BotArgs),


    /// Commands to create and manipulate K8s Job Workloads
    Workload(WorkloadArgs),

    /// Information about the command-line application
    Info
}

/// FlatBoat - CLI for robotics containerized docker environment
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    /// Subcommand Category
    #[command(subcommand)]
    pub command: Commands,
}