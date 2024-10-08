use serde_derive::Serialize;
use thiserror::Error;

use super::creation::provision::result::ProvisionError;

pub type PackageResult = Result<PackageOutput, PackageError>;

/// Package subcommand error type
#[derive(Debug, Error)]
pub enum PackageErrorType {
    #[error("Unable to create a devcontainer")]
    DevcontainerError,
    #[error("Unable to create the package")]
    PackageCreationError,
    #[error("Feature not implemented!")]
    NotImplemented,
    #[error("Unable to find the manifest")]
    ManifestNotFound,
    #[error("Unable to configure the package")]
    ConfigurationError,
    #[error("Unable to create the Dockerfile")]
    DockerfileError,

    #[error("Provision error: {0}")]
    ProvisionError(#[from] ProvisionError),
}

/// Package subcommand error information
#[derive(Serialize, Debug, Clone, Hash)]
pub struct PackageError {
    pub kind: PackageErrorType,
    pub desc: &'static str,
}

/// Package subcommand success output
#[derive(Serialize, Debug, Clone, Hash)]
pub struct PackageOutput {
    pub desc: &'static str,
}
