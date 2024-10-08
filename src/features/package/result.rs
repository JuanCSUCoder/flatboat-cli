use serde_derive::Serialize;
use subprocess::PopenError;
use thiserror::Error;

use super::creation::provision::result::ProvisionError;

pub type PackageResult = Result<PackageOutput, PackageError>;

/// Package subcommand error type
#[derive(Debug, Error)]
pub enum PackageError {
    #[error("Unable to create a devcontainer")]
    DevcontainerError,

    #[error("Unable to create the package")]
    SystemError(#[from] PopenError),

    #[error("Unable to create the package")]
    PackageCreationError,

    #[error("Feature not implemented!")]
    NotImplemented,

    #[error("Provision error: {0}")]
    ProvisionError(#[from] ProvisionError),
}

/// Package subcommand success output
#[derive(Serialize, Debug, Clone, Hash)]
pub struct PackageOutput {
    pub desc: &'static str,
}
