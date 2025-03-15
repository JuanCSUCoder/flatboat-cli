use std::{any::Any, ffi::CStr};

use pyo3_ffi::c_str;
use serde_json::Map;
use thiserror::Error;
use pyo3::{types::{PyAnyMethods, PyDict, PyModule}, IntoPyObject, Py, PyAny, PyErr, Python};

const ROCKER_INTERFACE_SRC: &CStr = c_str!(include_str!("./rocker_interface.py"));

#[derive(Error, Debug)] 
enum RockerSetupError {
  #[error("Failed loading Flatboat-Rocker Interface. PLEASE REPORT THIS BUG IN GITHUB.")]
  ErrorLoadingInterface(#[from] PyErr),
}

/// Setups and mantains the required environment for running a Rocker container
pub async fn get_rocker_config(extension_modules: Vec<String>, arguments: Map<String, Box<dyn Any>>) -> Result<(), RockerSetupError> {
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

struct ValidMap (Map<String, Box<dyn Any>>);

impl IntoPyObject for ValidMap {
    type Target;

    type Output = PyDict;

    type Error;

    fn into_pyobject(self, py: Python<'_>) -> Result<Self::Output, Self::Error> {
        self.0
    }
}

impl ValidMap {
    pub fn from(map: Map<String, Box<dyn Any>>) -> Self {
      Self(map)
    }
}

