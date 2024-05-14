use std::{env, fs, path::PathBuf, process};

use subprocess::{Exec, ExitStatus, PopenError};

use crate::args;

/// Handles all workspace related commands
pub fn handle_ws_cmd(ws_cmd: args::WorkspaceSubcommands) {
    return match ws_cmd {
        args::WorkspaceSubcommands::Create { ws_name, ws_manifest } => load_from_manifest(ws_name, ws_manifest)
    }
}

fn load_from_manifest(ws_name: String, ws_manifest: Option<String>) {
    

    // Create the folder
    let path = create_ws_dir(&ws_name);

    // Download the manifest
    // TODO: Read manifest
    // TODO: Pull and install devcontainer
}

fn create_ws(ws_name: String, ws_image: Option<String>) {
    let path = create_ws_dir(&ws_name);

    match env::set_current_dir(path) {
        Ok(_) => info!("Entering Workspace ..."),
        Err(_) => error!("Unable to access created folder {}", &ws_name),
    };

    let possible_urls = if let Some(image) = ws_image {
        vec![
            format!("ghcr.io/JuanCSUCoder/flatboat-templates/roboten_ws_{}", &image),
            format!("ghcr.io/JuanCSUCoder/flatboat-templates/{}", &image),
            format!("ghcr.io/JuanCSUCoder/{}", &image),
            format!("ghcr.io/{}", &image),
            format!("{}", &image),
        ]
    } else {
        vec![
            "ghcr.io/JuanCSUCoder/flatboat-templates/roboten_ws_iron_nogpu".to_string()
        ]
    };

    let mut success = false;
    for possible_url in possible_urls {
        info!("Trying to pull from {} ...", &possible_url);

        let res = create_ws_files(&possible_url).unwrap();

        if res.success() {
            success = true;
            break;
        }
    }

    if success {
        info!("Workspace Created Successfully!");
    } else {
        error!("Failed to create the workspace! Make sure the provided template image is correct.");
    }
}

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
fn create_ws_files(image_url: &String) -> Result<ExitStatus, PopenError>{
    Exec::cmd("devcontainer")
        .args(&[
            "templates",
            "apply",
            "-t",
            &image_url,
        ])
        .join()
}
