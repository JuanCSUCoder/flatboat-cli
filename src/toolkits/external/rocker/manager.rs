use serde_json::Map;

use crate::toolkits::external::rocker::{runtime, serde_pyo3};

#[derive(Debug, thiserror::Error)]
pub enum RockerConfigError {
    #[error("Failed to load the python Rocker interface. Please report this bug on GitHub.: {0}")]
    PythonInterfaceError(#[from] pyo3::PyErr),
    #[error("An error occurred while generating the Rocker configuration: {0}")]
    ConfigGenerationError(String),
    #[error("Failed to read the rocker config file: {0}")]
    FlagsReadError(#[from] std::io::Error),
    #[error("Failed to parse the rocker config file: {0}")]
    FlagsParseError(#[from] serde_json::Error),
    #[error("Invalid rocker configuration format: {0}")]
    InvalidFlagsFormat(String),
}

pub async fn configure_rocker() -> Result<(), RockerConfigError> {
    // 1. Get Rocker flags from .rocker/config.json
    let devcont_configuration_file = std::fs::read_to_string(".devcontainer/devcontainer.json")?;

    let dcconfig_val = serde_json::from_str::<serde_json::Value>(&devcont_configuration_file)?;
    let dconfig_map = dcconfig_val.as_object()
        .ok_or_else(|| RockerConfigError::InvalidFlagsFormat("Invalid devcontainer configuration format".to_string()))?;
    let flags: Map<String, serde_json::Value> = dconfig_map.clone();

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

/// Writes the generated Rocker configuration to the devcontainer.json file
fn write_devcontainer(rocker_config: &(String, Vec<String>)) -> Result<(), RockerConfigError> {
  return Ok(());
}

/// Writes the Dockerfile based on the generated Rocker configuration
fn write_dockerfile(rocker_config: &(String, Vec<String>)) -> Result<(), RockerConfigError> {
  return Ok(());
}