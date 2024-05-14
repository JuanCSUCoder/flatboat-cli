pub enum PullError {
	DownloadError(reqwest::Error),
	ParseError(toml::de::Error),
	NotFoundError,
	UnknownError,
}

pub trait Pullable {
    async fn pull(locator: String) -> Result<Self, PullError> where Self: Sized;

    async fn pull_or_default(locator: Option<String>) -> Result<Self, PullError> where Self: Sized;
}

impl From<reqwest::Error> for PullError {
	fn from(value: reqwest::Error) -> Self {
		PullError::DownloadError(value)
	}
}

impl From<toml::de::Error> for PullError {
	fn from(value: toml::de::Error) -> Self {
		PullError::ParseError(value)
	}
}