use serde_derive::Serialize;

use crate::utils::{manifest::Manifest, pull::PullError};

pub type ProgramResult = Result<ProgramOutput, ProgramError>;

#[derive(Serialize)]
pub enum ProgramOutput {
    WSCreate(Manifest),
	Ok,
    NoOutput,
}

#[derive(Serialize)]
pub enum ProgramError {
    WSCreate(PullError),
	ROSError,
	CommandError,
	DevcontainerError,
    UnknownError,
}
