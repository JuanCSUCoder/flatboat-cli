use crate::utils::{manifest::Manifest, pull::PullError};

pub enum ProgramOutput {
  WSCreate(Manifest),
  NoOutput,
}

pub enum ProgramError {
  WSCreate(PullError),
  UnknownError,
}