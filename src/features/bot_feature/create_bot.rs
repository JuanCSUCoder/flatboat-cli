use std::{env, path::PathBuf};

use subprocess::PopenError;
use thiserror::Error;

use crate::{toolkits, utils::manifest::{result::ManifestError, Manifest}};

#[derive(Error, Debug)]
pub enum CreateBotError {
  #[error("Error obtaining workspace manifest: {0}")]
  WorkspaceError(#[from] ManifestError),

  #[error("Workspace not found")]
  WorkspaceNotFound,

  #[error("Unable to create bot folder: {0}")]
  IOError(#[from] std::io::Error),

  #[error("Unable to pull bot image: {0}")]
  PullError(#[from] PopenError)
}

/// Creates a bot in workspace
pub fn create_bot(bot_name: &str) -> Result<(), CreateBotError> {
  // Get current workspace Manifest
	let manifest = Manifest::new()?;

  let pkg_path = PathBuf::from(manifest.ws_path.ok_or(CreateBotError::WorkspaceNotFound)?)
    .join("bots")
    .join(bot_name);

  // Moves inside the package direcctory
	env::set_current_dir(pkg_path)?;

	// Adds Docker File Configuration
	toolkits::oras::pull_template(&manifest.artifacts.bot)?;

  return Ok(());
}
