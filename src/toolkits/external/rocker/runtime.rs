use std::{any::Any, collections::HashMap, ffi::CStr};

use pyo3_ffi::c_str;
use thiserror::Error;
use pyo3::{types::PyModule, Python, PyErr};

const ROCKER_INTERFACE_SRC: &CStr = c_str!(include_str!("./rocker-interface.py"));

#[derive(Error, Debug)] 
enum RockerSetupError {
  #[error("Failed loading Flatboat-Rocker Interface. PLEASE REPORT THIS BUG IN GITHUB.")]
  ErrorLoadingInterface(#[from] PyErr),
}

/// Setups and mantains the required environment for running a Rocker container
pub async fn setup_environment(extension_modules: Vec<String>, arguments: HashMap<String, Box<dyn Any>>) -> Result<(), RockerSetupError> {
  Python::with_gil(|py| {
    let function = PyModule::from_code(
      py,
      ROCKER_INTERFACE_SRC,
      c_str!("rocker-interface.py"),
      c_str!("rocker_interface")
    )?;

    Ok(())
  })
}