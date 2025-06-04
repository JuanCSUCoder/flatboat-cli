use std::collections::HashMap;

use serde_json::Map;

use crate::toolkits::external::rocker::{runtime, serde_pyo3};

use super::model::DevcontainerConfig;

#[derive(Debug, thiserror::Error)]
pub enum RockerConfigError {
    #[error("\n\nPossible Solutions: \n \t- Please check if you have rocker and it's dependencies properly installed with pip directly on your system. Flatboat doesn't detect Anaconda nor VirtualEnv installations. \n \t- You may also have a conda or venv environment activated, please deactivate it and try again. \n\nFailed to load the python Rocker interface.: {0}")]
    PythonInterfaceError(#[from] pyo3::PyErr),
    #[error("\n\nNON-COMPATIBLE TEMPLATE: Since Flatboat v0.5.0 you need to use agnostic rocker compatible templates. \n\nFailed to read the rocker config file: {0}")]
    FlagsReadError(#[from] std::io::Error),
    #[error("Failed to parse the rocker config file: {0}")]
    FlagsParseError(#[from] serde_json::Error),
    #[error("Invalid rocker configuration format: {0}")]
    InvalidFlagsFormat(String),

    #[error("Failed to configure devcontainer: {0}")]
    DevcontainerJSONError(#[from] DevcontainerConfigError),

    #[error("Failed to write the Dockerfile: {0}")]
    DockerfileError(#[from] DockerfileError),
}

pub async fn configure_rocker() -> Result<(), RockerConfigError> {
    // 1. Get Rocker flags from .rocker/config.json
    let rocker_configuration_file = std::fs::read_to_string(".rocker/config.json")?;

    let rocker_val = serde_json::from_str::<serde_json::Value>(&rocker_configuration_file)?;
    let rocker_map = rocker_val.as_object()
        .ok_or_else(|| RockerConfigError::InvalidFlagsFormat("Invalid devcontainer configuration format".to_string()))?;
    let flags: Map<String, serde_json::Value> = rocker_map.clone();

    // 2. Load the Rocker interface, generate the configuration and write it to the files
    populate_rocker_configuration(flags).await?;

    return Ok(());
}

/// Fills the devcontainer configuration with rocker auto-generated values
async fn populate_rocker_configuration(flags: Map<String, serde_json::Value>) -> Result<(), RockerConfigError> {
  // TODO: 1. Load extra rocker extensions

  // 2. Generate the rocker configuration
  let rocker_config = runtime::get_rocker_config(
    vec![], // TODO: This should be replaced with actual extension modules
    serde_pyo3::ValidMap::from(flags), // This should be replaced with actual arguments
  ).await?;

  // 3. Write the configuration to the devcontainer.json file
  write_devcontainer(&rocker_config)?;

  // 4. Write Dockerfile
  write_dockerfile(&rocker_config)?;

  return Ok(());
}

#[derive(Debug, thiserror::Error)]
pub enum DevcontainerConfigError {
    #[error("Failed to read the devcontainer.json file: {0}")]
    DevcontainerIOError(#[from] std::io::Error),
    #[error("Failed to parse the devcontainer.json file: {0}")]
    DevcontainerParseError(#[from] serde_json::Error),
    #[error("Failed to detect current user. Please ensure $USER variable is set: {0}")]
    UserDetectionError(#[from] std::env::VarError),
}

/// Writes the generated Rocker configuration to the devcontainer.json file
fn write_devcontainer(rocker_config: &(String, Vec<String>)) -> Result<(), DevcontainerConfigError> {
  // 1. Read the existing devcontainer.json file
  let devcontainer_str = std::fs::read_to_string(".devcontainer/devcontainer.json")?;

  let mut devcontainer_config: DevcontainerConfig = serde_json::from_str(&devcontainer_str)?;
  let username = std::env::var("USER")?;

  // 2. Configure arguments
  devcontainer_config.privileged = true;
  devcontainer_config.container_env = HashMap::new();
  devcontainer_config.run_args = rocker_config.1.clone();
  devcontainer_config.mounts = vec![];
  devcontainer_config.remote_user = username.clone();

  // 3. Configure Dockerfile
  devcontainer_config.build.dockerfile = "Dockerfile".to_string();
  devcontainer_config.build.context = ".".to_string();
  devcontainer_config.build.args = {
    let mut args = HashMap::new();
    args.insert("USERNAME".to_string(), username);
    args
  };

  // 4. Write the updated configuration back to the file
  let updated_devcontainer_str = serde_json::to_string_pretty(&devcontainer_config)?;
  std::fs::write(".devcontainer/devcontainer.json", updated_devcontainer_str)?;

  return Ok(());
}


#[derive(Debug, thiserror::Error)]
pub enum DockerfileError {
  #[error("Failed to write the Dockerfile: {0}")]
  DockerfileWriteError(#[from] std::io::Error),
}

/// Writes the Dockerfile based on the generated Rocker configuration
fn write_dockerfile(rocker_config: &(String, Vec<String>)) -> Result<(), DockerfileError> {
  std::fs::write(
    ".devcontainer/Dockerfile",
    rocker_config.0.clone()
  )?;
  return Ok(());
}