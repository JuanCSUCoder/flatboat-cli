mod args;

use args::Cli;
use clap::Parser;
use subprocess::Exec;

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        args::Commands::Workspace(ws_args) => match ws_args.subcommand {
            args::WorkspaceSubcommands::Create { ws_name } => {
                println!("Creating Workspace {} ...", ws_name);
                Exec::cmd("docker").arg("info").join().unwrap();
                todo!()
            },
            args::WorkspaceSubcommands::List => {
                println!("Available Workspaces");
                todo!();
            },
            args::WorkspaceSubcommands::Delete { ws_name } => {
                println!("Deleting Workspace {} ...", ws_name);
                todo!();
            },
        },
        args::Commands::Info => println!("FlatBoat is a command-line interface application used to access, configure and manage dockerized ROS2 development environments, and for interfacing with ros2 cli"),
    }
}
