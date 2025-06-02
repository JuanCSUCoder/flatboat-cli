use std::io;

use serde_derive::Serialize;
use subprocess::PopenError;
use thiserror::Error;

use crate::{toolkits::jinja::TemplatingError, utils::result::PackageConfigError};

use super::{creation::provision::result::ProvisionError, pkg_build::BuildError};

pub type PackageResult = Result<PackageOutput, PackageError>;

/// Package subcommand error type
#[derive(Debug, Error)]
pub enum PackageError {
    #[error("Unable to create a devcontainer")]
    DevcontainerError,

    #[error("Unable to create the package: {0}")]
    SystemError(#[from] PopenError),

    #[error("Unable to create the package")]
    PackageCreationError,

    #[error("Provision error: {0}")]
    ProvisionError(#[from] ProvisionError),

    #[error("Unable to build package")]
    PackageBuildError(#[from] BuildError),

    #[error("Unable to resolve absolute path for the current workspace")]
    PathResolutionError(#[from] io::Error),

    #[error("Unable to read package configuration: {0}")]
    PackageConfigError(#[from] PackageConfigError),

    #[error("Failed to process template: {0}")]
    TemplatingError(#[from] TemplatingError),
}

/// Package subcommand success output
#[derive(Serialize, Debug, Clone, Hash)]
pub struct PackageOutput {
    pub desc: &'static str,
}
