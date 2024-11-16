use subprocess::PopenError;
use thiserror::Error;

use crate::utils::manifest::result::ManifestError;

use super::pkg_ops::result::PkgOpsError;

#[derive(Debug, Error)]
pub enum ProvisionError {
  #[error("Unable to get workspace manifest: {0}")]
  WsManifestError(#[from] ManifestError),

  #[error("IO Error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Sub-process system error: {0}")]
  SystemError(#[from] PopenError),

  #[error("Failed to provision package: {0}")]
  TemplateProvisionError(#[from] PkgOpsError),
}