use std::{any::Any, ffi::CStr};

use pyo3_ffi::{c_str, PyErr_BadArgument};
use thiserror::Error;
use pyo3::{types::{PyAnyMethods, PyDict, PyInt, PyModule, PyString}, Bound, IntoPyObject, Py, PyAny, PyErr, Python};

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
pub async fn get_rocker_config(extension_modules: Vec<String>, arguments: ValidMap) -> Result<(), RockerSetupError> {
  Python::with_gil(|py| {
    let function: Py<PyAny> = PyModule::from_code(
      py,
      ROCKER_INTERFACE_SRC,
      c_str!("rocker_interface.py"),
      c_str!("rocker_interface")
    )?
    .getattr("get_rocker_config")?
    .into();

    let args = (extension_modules, arguments);

    function.call1(py, args)?;

    Ok(())
  })
}

mod tests {
  use super::*;
  use crate::toolkits::external::rocker::serde_pyo3::ValidMap;
  use serde_json::json;

  #[tokio::test]
  async fn test_get_rocker_config() -> Result<(), RockerSetupError> {
    let extension_modules = vec!["rocker".to_string()];
    let json = json!({
      "key1": "value1",
      "key2": "value2"
    });
    let obj = json.as_object().ok_or(RockerSetupError::TestError)?;
    let arguments = ValidMap::from(obj.clone());

    get_rocker_config(extension_modules, arguments).await?;

    Ok(())
  }
}

