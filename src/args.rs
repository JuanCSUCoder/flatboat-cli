use clap::{Parser, Subcommand, Args};

#[derive(Subcommand)]
pub enum WorkspaceSubcommands {
    /// Creates a new workspace in the specified location
    Create {
        /// Name of the workspace to be created
        ws_name: String,
    }
}

#[derive(Subcommand)]
pub enum BotSubcommands {
    /// Creates a dockerized K8s Node for ROS2 Robots
    Create {
        /// Name of the bot to be created
        bot_name: String
    },

    /// Launch the dockerized K8s Node in the Kubernetes Cluster
    BringUp {
        /// Name of the bot to be started
        bot_name: String
    },

    /// Removes the dockerized K8s Node from the Kubernetes Cluster
    BringDown {
        /// Name of the bot to be stopped
        bot_name: String
    },

    /// Re-launch the dockerized K8s Node in the Kubernetes Cluster
    Refresh {
        /// Name of the bot to be re-launched
        bot_name: String
    },
}

#[derive(Subcommand)]
pub enum WorkloadSubcommands {
    /// Create a K8s Job Workload for a ROS2 Robot
    Create {
        /// Name of the workload to be created
        wl_name: String
    },

    /// Launch a K8s Job Workload in the Current Kubernetes Cluster for a ROS2 Robot
    Deploy {
        /// Name of the workload to be deployed
        wl_name: String
    },
}

#[derive(Subcommand)]
pub enum PackageSubcommands {
    /// Creates a new ROS2 Package
    Create {
        /// Name of the package to be created
        pkg_name: String
    },

    /// Builds an existing ROS2 Package
    Build {
        /// Name of the package to be built
        pkg_name: String
    }
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
    pub subcommand: WorkloadSubcommands,
}

/// Package Subcommands
#[derive(Args)]
pub struct PackageArgs {
    #[command(subcommand)]
    pub subcommand: PackageSubcommands,
}

/// Shell Completition Generation Subcommand
#[derive(Args)]
pub struct GeneratorArgs {
    // If provided, outputs the completion file for given shell
    pub generator: clap_complete::Shell,
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

    /// Commands to create and build ROS2 packages
    Package(PackageArgs),

    /// Generate Shell Completitions
    Generator(GeneratorArgs),

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