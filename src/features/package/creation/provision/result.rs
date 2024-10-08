use thiserror::Error;

use crate::utils::result::PackageConfigError;

#[derive(Debug, Error)]
pub enum PkgOpsError {
  variant,
  variant2,
}

#[derive(Debug, Error)]
pub enum UpdatePackageConfigError {
  #[error("Unable to get package configuration: {0}")]
  PkgConfig(#[from] PackageConfigError),
  
  #[error("Unable to serialize package configuration: {0}")]
  SerializationError(#[from] toml::ser::Error),

  #[error("Unable to write to file: {0}")]
  IoError(#[from] std::io::Error)
}