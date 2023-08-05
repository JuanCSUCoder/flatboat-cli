use clap::{Parser, Subcommand, Args};

#[derive(Subcommand)]
enum WorkspaceSubcommands {
    /// Creates a new workspace in the specified location
    Create {
        #[arg(short, long)]
        ws_name: String,
    },

    /// Lists existing workspaces
    List,

    /// Deletes an existing workspace
    Delete {
        #[arg(short, long)]
        ws_name: String
    },
}

/// Workspace Subcommands
#[derive(Args)]
struct WorkspaceArgs {

    /// Workspace Sub-commands
    #[command(subcommand)]
    subcommand: WorkspaceSubcommands,
}

/// Commands
#[derive(Subcommand)]
enum Commands {
    /// Commands to create and manipulate a Dockerized ROS2 Workspace
    Workspace(WorkspaceArgs),

    /// Information about the command-line application
    Info
}

/// FlatBoat - CLI for robotics containerized docker environment
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {

    /// Subcommand Category
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let _cli = Cli::parse();
    println!("Hello, world!");
}
