use serde_derive::Serialize;

use crate::{features::package::result::{PackageError, PackageOutput}, utils::{manifest::Manifest, pull::PullError}};

pub type ProgramResult = Result<ProgramOutput, ProgramError>;

#[derive(Serialize)]
#[serde(tag = "output")]
pub enum ProgramOutputKind {
    WSCreate(Manifest),
    PKGCreate(PackageOutput),
	Ok,
    NoOutput,
}

/// Program output information
#[derive(Serialize)]
pub struct ProgramOutput {
    pub kind: ProgramOutputKind,
    pub desc: &'static str,
}

#[derive(Serialize)]
pub enum ProgramErrorKind {
    WSCreate(PullError),
    PKGCreate(PackageError),
	ROSError,
	CommandError,
	DevcontainerError,
    UnknownError,
}

/// Program error information
#[derive(Serialize)]
pub struct ProgramError {
    pub kind: ProgramErrorKind,
    pub desc: &'static str,
}
