use std::ffi::CStr;

use pyo3_ffi::c_str;
use thiserror::Error;
use pyo3::{types::{PyAnyMethods, PyModule}, PyErr, Python};

use crate::toolkits::external::rocker;

use super::ValidMap;

const ROCKER_INTERFACE_SRC: &CStr = c_str!(include_str!("./rocker_interface.py"));

#[derive(Error, Debug)] 
pub enum RockerSetupError {
  #[error("Failed loading Flatboat-Rocker Interface. PLEASE REPORT THIS BUG IN GITHUB.")]
  ErrorLoadingInterface(#[from] PyErr),

  #[error("Test error. PLEASE REPORT THIS BUG IN GITHUB.")]
  TestError,
}

/// Setups and mantains the required environment for running a Rocker container
pub async fn get_rocker_config(extension_modules: Vec<String>, arguments: ValidMap) -> Result<(String, String), PyErr> {
  debug!("Generating rocker config...");
  Python::with_gil(|py| {
    debug!("Started global interpreter lock!");

    let rocker_module = PyModule::from_code(
      py,
      ROCKER_INTERFACE_SRC,
      c_str!("rocker_interface.py"),
      c_str!("rocker_interface")
    )?;

    debug!("Rocker interface loaded");

    let function = rocker_module.getattr("get_rocker_config")?;

    debug!("Preparing to generate rocker config with: {:?}", function);

    let args = (extension_modules, arguments);

    let result = function.call1(args)?;

    debug!("Rocker Config Generated!");

    return rocker::py_mapper::map2rs(result);
  })
}

mod tests {
  use super::*;
  use crate::toolkits::external::rocker::serde_pyo3::ValidMap;
  use pyo3::exceptions::PyValueError;
  use serde_json::json;

  #[tokio::test]
  async fn test_get_rocker_config() -> Result<(), PyErr> {
    crate::core::helpers::setup_logging();
    let extension_modules: Vec<String> = Vec::new();
    let json = json!({
      "base_image": "ubuntu:22.04",

      "x11": true,

      "nvidia": true,
      "cuda": true,

      "git": true,
      "user": true,
      "network": "host",
      "privileged": true,
      "pulse": true,
    });
    let obj = json.as_object().ok_or(PyErr::new::<PyValueError, _>("Error"))?;
    let arguments = ValidMap::from(obj.clone());

    get_rocker_config(extension_modules, arguments).await?;

    Ok(())
  }

  #[tokio::test]
  async fn test_python_environment() -> Result<(), PyErr> {
    Python::with_gil(|py| {
      py.run(c_str!(r#"
import sys
print("SYS.VERSION: ", sys.version)
print("SYS.EXEC: ", sys.executable)
print("SYS.PATH: ", sys.path)

import rocker
print("ROCKER.READY: ", rocker.__name__)
      "#), None, None)?;

      return Ok(());
    })
  }
}
