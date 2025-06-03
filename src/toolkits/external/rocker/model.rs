use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevcontainerConfig {
  #[serde(default)]
  pub name: String,

  #[serde(default = "default_true")]
  pub privileged: bool,

  #[serde(default)]
  pub build: BuildConfig,
  pub workspace_folder: Option<String>,
  pub workspace_mount: Option<String>,
  pub container_env: HashMap<String, String>,
  pub run_args: Vec<String>,
  pub mounts: Vec<String>,

  #[serde(default)] // This ensures the HashMap defaults to empty if no extra fields
  #[serde(flatten)] // This tells Serde to "flatten" all other fields into this map
  pub extra_fields: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildConfig {
  #[serde(default = "default_dockerfile")]
  pub dockerfile: String,
  #[serde(default = "default_context")]
  pub context: String,
  #[serde(default)] // This ensures the HashMap defaults to empty if no args are provided
  pub args: HashMap<String, String>,

  #[serde(default)] // This ensures the HashMap defaults to empty if no extra fields
  #[serde(flatten)] // This tells Serde to "flatten" all other fields into this map
  pub extra_fields: HashMap<String, serde_json::Value>,
}

impl Default for BuildConfig {
  fn default() -> Self {
    BuildConfig {
      dockerfile: "Dockerfile".to_string(),
      context: ".".to_string(),
      args: HashMap::new(),
      extra_fields: HashMap::new(),
    }
  }
}

// Helper functions for serde default
fn default_true() -> bool {
  true
}

fn default_dockerfile() -> String {
  "Dockerfile".to_string()
}

fn default_context() -> String {
  ".".to_string()
}
