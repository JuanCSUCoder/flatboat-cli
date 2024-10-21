use std::fmt::Display;

use serde_derive::Serialize;

/// Manifest Error Description
#[derive(Serialize, Debug, Clone, Hash)]
pub struct ManifestError {
  pub desc: &'static str,
}

impl Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error Description: {}", self.desc)
    }
}

impl std::error::Error for ManifestError {}
