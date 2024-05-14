use std::{fs::{self, File}, io::Write, path::PathBuf, process};

use subprocess::{Exec, ExitStatus, PopenError};

use crate::{args, utils::{self, pull::Pullable}};

/// Handles all workspace related commands
pub async fn handle_ws_cmd(ws_cmd: args::WorkspaceSubcommands) -> Result<utils::manifest::Manifest, utils::pull::PullError> {
    return match ws_cmd {
        args::WorkspaceSubcommands::Create { ws_name, ws_manifest } => load_from_manifest(ws_name, ws_manifest).await
    }
}

async fn load_from_manifest(ws_name: String, ws_manifest: Option<String>) -> Result<utils::manifest::Manifest, utils::pull::PullError> {
    // Create the folder
    let mut path = create_ws_dir(&ws_name);

    // Download the manifest
    let manifest = utils::manifest::Manifest::pull_or_default(ws_manifest).await?;
    
    // Pull and install devcontainer
    create_ws_files(&manifest.artifacts.workspace);

    // Install the manifest inside the workspace
    path.push("flatboat.toml");
    let mut manifest_file = File::create(path)?;
    manifest_file.write_all(toml::to_string_pretty(&manifest)?.as_bytes());

    Ok(manifest)
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
