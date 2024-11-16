use serde::{ser::SerializeSeq, Serialize};
use serde_derive::Serialize;
use thiserror::Error;

use crate::{features::{package::result::{PackageError, PackageOutput}, workload::WorkloadCmdError}, utils::{manifest::Manifest, pull::PullError}};

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

#[derive(Debug, Error)]
pub enum ProgramErrorKind {
    #[error("Unable to create workspace")]
    WSCreate(PullError),

    #[error("Unable to process package command: {0}")]
    PKG(#[from] PackageError),

    #[error("Unable to process create workload command: {0}")]
    WL(WorkloadCmdError),

    #[error("Error with ROS command")]
	ROSError,

    #[error("Error with command execution")]
	CommandError,

    #[error("Error with devcontainer")]
	DevcontainerError,

    #[error("Unknown Error")]
    UnknownError,
}

/// Program error information
pub struct ProgramError {
    pub kind: ProgramErrorKind,
    pub desc: &'static str,
}

impl Serialize for ProgramError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.desc)?;
        seq.serialize_element(&self.kind.to_string())?;

        seq.end()

    }
}
