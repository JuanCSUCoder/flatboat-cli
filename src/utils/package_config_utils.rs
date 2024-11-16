use std::{fs, path::Path};

use serde_derive::{Deserialize, Serialize};

use super::result::PackageConfigError;

/// Package configuration file contained in pkg.toml
#[derive(Serialize, Deserialize)]
pub struct PackageConfig {
  pub package_name: String,
  pub system_dependencies: Vec<String>,
  pub launch_file: Option<String>,
  pub command_file: Option<String>,
  pub extra_args: Option<String>
}

impl PackageConfig {
    /// Constructs a package configuration object from working directory pkg.toml
    pub fn from_current_folder() -> Result<Self, PackageConfigError> {
      let file_content = fs::read_to_string(Path::new("pkg.toml"))?;
      
      return Ok(toml::de::from_str::<Self>(&file_content)?);
    }

    pub fn from_path(pkg_path: &Path) -> Result<Self, PackageConfigError> {
      let file_content = fs::read_to_string(pkg_path.join("pkg.toml"))?;

      return Ok(toml::de::from_str(&file_content)?);
    }
}
