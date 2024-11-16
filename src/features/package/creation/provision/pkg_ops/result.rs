use thiserror::Error;

use crate::utils::result::PackageConfigError;

#[derive(Debug, Error)]
pub enum PkgOpsError {
  #[error("Unable to update package configuration: {0}")]
  UpdateConfigError(#[from] UpdatePackageConfigError),

  #[error("Unable to apply dockerfile template: {0}")]
  TemplatingError(#[from] ApplyDockerfileTemplateError)
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

#[derive(Debug, Error)]
pub enum ApplyDockerfileTemplateError {
  #[error("Unable to read template file: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Unable to get package configuration: {0}")]
  PkgConfig(#[from] PackageConfigError),

  #[error("Unable to render template: {0}")]
  RenderTemplateError(#[from] minijinja::Error),
}
