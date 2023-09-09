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
    /// Creates a dockerized K8s Node for ROS2 Robots
    Create,

    /// Launch the dockerized K8s Node in the Kubernetes Cluster
    BringUp,

    /// Removes the dockerized K8s Node from the Kubernetes Cluster
    BringDown,

    /// Re-launch the dockerized K8s Node in the Kubernetes Cluster
    Refresh,
}

#[derive(Subcommand)]
pub enum WorkloadSubcommands {
    /// Create a K8s Job Workload for a ROS2 Robot
    Create,

    /// Launch a K8s Job Workload in the Current Kubernetes Cluster for a ROS2 Robot
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