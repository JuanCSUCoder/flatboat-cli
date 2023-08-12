mod args;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::{fs, process, path::Path};

use args::Cli;
use clap::Parser;
use subprocess::Exec;

fn main() {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Info).init();
    let cli = Cli::parse();
    
    match cli.command {
        args::Commands::Workspace(ws_args) => match ws_args.subcommand {
            args::WorkspaceSubcommands::Create { ws_name } => {
                info!("Creating Workspace {} ...", &ws_name);
                let path = Path::new(&ws_name);
                match fs::create_dir(&path) {
                    Ok(_) => info!("Folder {} created at {:?}", &ws_name, path.canonicalize().unwrap()),
                    Err(e) => {
                        error!("Unable to create workspace folder {} at {:?}: {}", &ws_name, path.canonicalize(), e);
                        process::exit(1);
                    },
                };
                // Exec::cmd("docker").arg("info").join().unwrap();
                error!("Not Implemented");
            },
            args::WorkspaceSubcommands::List => {
                info!("Available Workspaces");
                todo!();
            },
            args::WorkspaceSubcommands::Delete { ws_name } => {
                info!("Deleting Workspace {} ...", ws_name);
                todo!();
            },
        },
        args::Commands::Info => info!("FlatBoat is a command-line interface application used to access, configure and manage dockerized ROS2 development environments, and for interfacing with ros2 cli"),
    }
}
