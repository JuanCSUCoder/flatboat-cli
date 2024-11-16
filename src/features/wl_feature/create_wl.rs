use std::{env, path::PathBuf};

use subprocess::PopenError;
use thiserror::Error;

use crate::{toolkits, utils::manifest::{result::ManifestError, Manifest}};

#[derive(Error, Debug)]
pub enum CreateWorkloadError {
  #[error("Error obtaining workspace manifest: {0}")]
  WorkspaceError(#[from] ManifestError),

  #[error("Workspace not found")]
  WorkspaceNotFound,

  #[error("Unable to create wl folder: {0}")]
  IOError(#[from] std::io::Error),

  #[error("Unable to pull wl image: {0}")]
  PullError(#[from] PopenError)
}

/// Creates a wl in workspace
pub fn create_wl(wl_name: &str) -> Result<(), CreateWorkloadError> {
  // Get current workspace Manifest
	let manifest = Manifest::new()?;

  let pkg_path = PathBuf::from(manifest.ws_path.ok_or(CreateWorkloadError::WorkspaceNotFound)?)
    .join("wl")
    .join(wl_name);

  // Moves inside the package direcctory
	env::set_current_dir(pkg_path)?;

	// Adds Docker File Configuration
	toolkits::oras::pull_template(&manifest.artifacts.workload)?;

  return Ok(());
}
