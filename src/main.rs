mod args;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use args::Cli;
use clap::Parser;
use subprocess::Exec;

fn main() {
    pretty_env_logger::init();
    let cli = Cli::parse();
    
    match cli.command {
        args::Commands::Workspace(ws_args) => match ws_args.subcommand {
            args::WorkspaceSubcommands::Create { ws_name } => {
                println!("Creating Workspace {} ...", ws_name);
                Exec::cmd("docker").arg("info").join().unwrap();
                error!("Not Implemented");
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
