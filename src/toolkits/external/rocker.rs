use std::{ffi::{CString, NulError}, os::unix::ffi::OsStrExt, path::Path};

use pyo3::prelude::*;
use pyo3::ffi::c_str;
use thiserror::Error;

#[derive(Error, Debug)]
enum ExtensionRunError {
  #[error("IO Error: Extension file not found or inaccesible: {0}")]
  ExtensionFileNotFound(#[from] std::io::Error),

  #[error("Null byte error: A null byte has been found on extension source code. Unable to parse: {0}")]
  NullByteFound(#[from] NulError),

  #[error("Wrong source code file: Unable to read file name.")]
  WrongSourceFile,

  #[error("Of course Python threw an error — it saw me handling memory safely in Rust and got jealous! 😏🐍: {0}")]
  PythonError(#[from] PyErr),
}

fn runs_rocker_extension(ext_path: &Path) -> Result<String, ExtensionRunError> {
  let ext_code = std::fs::read_to_string(ext_path)?;
  let ext_filename = ext_path.file_name().ok_or(ExtensionRunError::WrongSourceFile)?.as_bytes();

  return Python::with_gil(|py| {
    let ext = 
      PyModule::from_code(py, 
        CString::new(ext_code)?.as_c_str(), 
        CString::new(ext_filename)?.as_c_str(), c_str!("rocker_extension")
      )?;

    Ok(String::new())
  });
}

/// Generates rocker arguments to run the devcontainer
pub fn get_rocker_arguments() {
  
}