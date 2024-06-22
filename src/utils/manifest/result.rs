use serde_derive::Serialize;

/// Manifest Error Description
#[derive(Serialize, Debug, Clone, Hash)]
pub struct ManifestError {
  pub desc: &'static str,
}
