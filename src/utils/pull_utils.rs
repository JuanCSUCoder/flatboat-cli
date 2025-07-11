use std::{error::Error, fmt::Display};

use serde::Serialize;
use serde_derive::Serialize;

use crate::toolkits::devcontainer::DevcontainerInitializationError;

#[derive(Debug)]
pub struct SerializableError(pub Box<dyn std::fmt::Debug>);

impl Serialize for SerializableError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{:?}", self.0.as_ref()).as_str())
    }
}

#[derive(Debug, Serialize)]
pub enum PullError {
    DownloadError(SerializableError),
    ParseError(SerializableError),
    SerializerError(SerializableError),
    WorkspaceAlreadyExistsError,
    DevcontainerError(DevcontainerInitializationError),
    NotFoundError,
    UnknownError,
}

impl Display for PullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PullError::DownloadError(e) => write!(f, "Download error: {:?}", e),
            PullError::ParseError(e) => write!(f, "Parse error: {:?}", e),
            PullError::SerializerError(e) => write!(f, "Serializer error: {:?}", e),
            PullError::WorkspaceAlreadyExistsError => write!(f, "Workspace already exists"),
            PullError::DevcontainerError(e) => write!(f, "Devcontainer error: {}", e),
            PullError::NotFoundError => write!(f, "Resource not found"),
            PullError::UnknownError => write!(f, "An unknown error occurred"),   
        }
    }
}

impl Error for PullError {}

pub trait Pullable {
    async fn pull(locator: String) -> Result<Self, PullError>
    where
        Self: Sized;

    async fn pull_or_default(locator: Option<String>) -> Result<Self, PullError>
    where
        Self: Sized;
}

impl From<reqwest::Error> for PullError {
    fn from(value: reqwest::Error) -> Self {
        PullError::DownloadError(SerializableError(Box::new(value)))
    }
}

impl From<toml::de::Error> for PullError {
    fn from(value: toml::de::Error) -> Self {
        PullError::ParseError(SerializableError(Box::new(value)))
    }
}

impl From<toml::ser::Error> for PullError {
    fn from(value: toml::ser::Error) -> Self {
        PullError::SerializerError(SerializableError(Box::new(value)))
    }
}

impl From<std::io::Error> for PullError {
    fn from(_value: std::io::Error) -> Self {
        PullError::UnknownError
    }
}

impl From<subprocess::PopenError> for PullError {
    fn from(_value: subprocess::PopenError) -> Self {
        PullError::UnknownError
    }
}

impl From<DevcontainerInitializationError> for PullError {
    fn from(value: DevcontainerInitializationError) -> Self {
        PullError::DevcontainerError(value)
    }
}

impl Serialize for DevcontainerInitializationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{:?}", self).as_str())
    }
}
