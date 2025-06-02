use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevcontainerConfig {
  pub name: String,
  pub privileged: bool,
  pub build: BuildConfig,
  pub workspace_folder: String,
  pub workspace_mount: String,
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
  pub dockerfile: String,
  pub context: String,
  pub args: HashMap<String, String>,

  #[serde(default)] // This ensures the HashMap defaults to empty if no extra fields
  #[serde(flatten)] // This tells Serde to "flatten" all other fields into this map
  pub extra_fields: HashMap<String, serde_json::Value>,
}