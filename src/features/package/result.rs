use serde_derive::Serialize;

pub type PackageResult = Result<PackageOutput, PackageError>;

/// Package subcommand error type
#[derive(Serialize, Debug, Clone, Hash)]
pub enum PackageErrorType {
    DevcontainerError,
    PackageCreationError,
    NotImplemented,
    ManifestNotFound,
    ConfigurationError,
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
