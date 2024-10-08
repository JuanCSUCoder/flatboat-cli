use serde_derive::Serialize;
use thiserror::Error;

use crate::{features::package::result::{PackageError, PackageOutput}, utils::{manifest::{result::ManifestError, Manifest}, pull::PullError}};

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

#[derive(Serialize, Debug, Error)]
pub enum ProgramErrorKind {
    WSCreate(PullError),
    PKGCreate(PackageError),
	ROSError,
	CommandError,
	DevcontainerError,
    ManifestErr(ManifestError),
    UnknownError,
}

/// Program error information
#[derive(Serialize)]
pub struct ProgramError {
    pub kind: ProgramErrorKind,
    pub desc: &'static str,
}
