use std::{io, path::Path};

use thiserror::Error;

use crate::utils::manifest::{result::ManifestError, Manifest};

#[derive(Debug, Error)] 
pub enum BuildError {
  #[error("Unable to resolve absolute path for the current workspace")]
  PathResolutionError(#[from] io::Error),

  #[error("Unable to get workspace manifest")]
  ManifestError(#[from] ManifestError),

  #[error("Error building docker image")]
  DockerBuildError(#[from] subprocess::PopenError),
}

/// Builds the selected package as a docker image
pub fn build_package(pkg_name: &str, ws: &Path, dockerfile: &Path) -> Result<(), BuildError> {
  let manifest = Manifest::new()?;

  crate::toolkits::docker::build_image(&ws, &dockerfile, &format!("flatboat-{}/{}", manifest.name, pkg_name))?;

  return Ok(());
}