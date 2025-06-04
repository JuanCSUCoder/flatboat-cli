use std::{env, fs::{self, File}, io::Write, path::{Path, PathBuf}};

use crate::{args, output::{ProgramError, ProgramErrorKind, ProgramOutput, ProgramOutputKind, ProgramResult}, toolkits::{devcontainer::create_ws_files, rocker}, utils::{self, manifest::Manifest, pull::{PullError, Pullable}}};

/// Handles all workspace related commands
pub async fn handle_ws_cmd(ws_cmd: args::WorkspaceSubcommands) -> ProgramResult {
    let res = match ws_cmd {
        args::WorkspaceSubcommands::Create { ws_name, ws_manifest } => load_from_manifest(ws_name, ws_manifest).await,
        args::WorkspaceSubcommands::Reconfigure => reconfigure_ws().await,
    };

    if let Ok(manifest) = res {
        Ok(ProgramOutput { kind: ProgramOutputKind::WSCreate(manifest), desc: "Success" })
    } else if let Err(error) = res {
        Err(ProgramError { kind: ProgramErrorKind::WSCreate(error), desc: "Unable to create workspace." })
    } else {
        Err(ProgramError { kind: ProgramErrorKind::UnknownError, desc: "Unknown error while creating workspace." })
    }
}

async fn load_from_manifest(ws_name: String, ws_manifest: Option<String>) -> Result<utils::manifest::Manifest, utils::pull::PullError> {
    // Create the folder
    let path = create_ws_dir(&ws_name)?;

    // Set current dir
    match env::set_current_dir(path) {
        Ok(_) => info!("Entering Workspace ..."),
        Err(_) => error!("Unable to access created folder {}", &ws_name),
    };

    // Download the manifest
    let manifest = utils::manifest::Manifest::pull_or_default(ws_manifest).await?;
    
    // Pull and install devcontainer
    create_ws_files(&manifest.artifacts.workspace).await?;

    // Install the manifest inside the workspace
    let file_path = Path::new("flatboat.toml");
    let mut manifest_file = File::create(file_path)?;
    manifest_file.write_all(toml::to_string_pretty(&manifest)?.as_bytes())?;

    Ok(manifest)
}

async fn reconfigure_ws() -> Result<Manifest, PullError> {
    info!("Reconfiguring Workspace ...");

    // Load the manifest
    let manifest: utils::manifest::Manifest = Manifest::new().map_err(|_| PullError::NotFoundError)?;

    // Pull and install devcontainer
    // Change to the workspace directory, return error if ws_path is None
    if let Some(ws_path) = &manifest.ws_path {
        std::env::set_current_dir(ws_path)
            .map_err(|_| PullError::NotFoundError)?;

        rocker::configure_rocker().await?;

        return Ok(manifest);
    } else {
        return Err(PullError::NotFoundError);
    }
}

/// Creates Workspace Directory
fn create_ws_dir(ws_name: &String) -> Result<PathBuf, PullError> {
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
            return Err(PullError::WorkspaceAlreadyExistsError);
        }
    };

    return Ok(path)
}

