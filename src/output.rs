use crate::utils::{manifest::Manifest, pull::PullError};

pub type ProgramResult = Result<ProgramOutput, ProgramError>;

pub enum ProgramOutput {
    WSCreate(Manifest),
	Ok,
    NoOutput,
}

pub enum ProgramError {
    WSCreate(PullError),
	ROSError,
	CommandError,
	DevcontainerError,
    UnknownError,
}
