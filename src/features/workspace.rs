use std::{env, fs, path::PathBuf, process};

use subprocess::{Exec, ExitStatus, PopenError};

use crate::args;

/// Creates Workspace Directory
fn create_ws_dir(ws_name: &String) -> PathBuf {
    info!("Creating Workspace {} ...", &ws_name);
    let path = PathBuf::from(ws_name);
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

    return path
}

/// Downloads the files from the Workspace Template
fn create_ws_files() -> Result<ExitStatus, PopenError>{
    Exec::cmd("devcontainer")
        .args(&[
            "templates",
            "apply",
            "-t",
            "ghcr.io/JuanCSUCoder/flatboat-templates/roboten_ws_iron_nogpu",
        ])
        .join()
}

fn create_ws(ws_name: String) {
    let path = create_ws_dir(&ws_name);

    match env::set_current_dir(path) {
        Ok(_) => info!("Entering Workspace ..."),
        Err(_) => error!("Unable to access created folder {}", &ws_name),
    };

    create_ws_files().expect("Error Creating Workspace Files!");

    info!("Workspace Created Successfully!");
}

/// Handles all workspace related commands
pub fn handle_ws_cmd(ws_cmd: args::WorkspaceSubcommands) {
    match ws_cmd {
        args::WorkspaceSubcommands::Create { ws_name } => create_ws(ws_name)
    }
}
