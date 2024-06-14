use serde_derive::Serialize;

pub type PackageResult = Result<PackageOutput, PackageError>;

/// Package subcommand error type
#[derive(Serialize)]
pub enum PackageErrorType {
    DevcontainerError,
    PackageCreationError,
    NotImplemented,
}

/// Package subcommand error information
#[derive(Serialize)]
pub struct PackageError {
    pub kind: PackageErrorType,
    pub desc: &'static str,
}

/// Package subcommand success output
#[derive(Serialize)]
pub struct PackageOutput {
    pub desc: &'static str,
}
