use std::{path::Path, process, env, fs};

use subprocess::Exec;

use crate::args;

/// Handles all workspace related commands
pub fn handle_ws_cmd(ws_cmd: args::WorkspaceSubcommands) {
    match ws_cmd {
        args::WorkspaceSubcommands::Create { ws_name } => {
            info!("Creating Workspace {} ...", &ws_name);
            let path = Path::new(&ws_name);
            match fs::create_dir(&path) {
                Ok(_) => info!(
                    "Folder {} created at {:?}",
                    &ws_name,
                    path.canonicalize().unwrap()
                ),
                Err(e) => {
                    error!(
                        "Unable to create workspace folder {} at {:?}: {}",
                        &ws_name,
                        path.canonicalize(),
                        e
                    );
                    process::exit(1);
                }
            };

            match env::set_current_dir(path) {
                Ok(_) => info!("Entering Workspace ..."),
                Err(_) => error!("Unable to access created folder {}", &ws_name),
            };
            Exec::cmd("devcontainer")
                .args(&[
                    "templates",
                    "apply",
                    "-t",
                    "ghcr.io/JuanCSUCoder/RobotEn/humble_nogpu",
                ])
                .join()
                .unwrap();

            info!("Workspace Created Successfully!");
        }
    }
}
