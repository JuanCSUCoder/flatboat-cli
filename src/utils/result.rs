use thiserror::Error;


#[derive(Debug, Error)]
pub enum PackageConfigError {
  #[error("I/O error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Toml deserialization error: {0}")]
  TomlError(#[from] toml::de::Error)
}