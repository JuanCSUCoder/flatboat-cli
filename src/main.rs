mod args;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::{fs, process, path::Path, env};

use args::Cli;
use clap::Parser;
use directories::ProjectDirs;
use subprocess::Exec;

fn main() {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Debug).init();
    let cli = Cli::parse();

    let project_dirs = ProjectDirs::from("codes", "juancsu", "flatboat");

    if project_dirs.is_none() {
        error!("Unable to locate application folders");
        process::exit(1);
    } else {
        debug!("Located folders {:?}", &project_dirs);
    }

    let project_dirs = project_dirs.unwrap();
    let data_dir = project_dirs.data_dir();

    debug!("Data Directory: {:?}", data_dir);
    
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

                match env::set_current_dir(path) {
                    Ok(_) => info!("Entering Workspace ..."),
                    Err(_) => error!("Unable to access created folder {}", &ws_name),
                };
                Exec::cmd("devcontainer").args(&["templates", "apply", "-t", "ghcr.io/JuanCSUCoder/RobotEn/humble_nogpu"]).join().unwrap();

                info!("Workspace Created Successfully!");
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
