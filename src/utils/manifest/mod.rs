use super::pull;

mod  locator;

/// Artifacts of a Flatboat Workspace
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct Artifacts {
	workspace: String,
	package: String,
	workload: String,
	bot: String,
}

/// Manifest of a Flatboat Workspace
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct Manifest {
	name: String,
	downloaded_from: Option<String>,
	artifacts: Artifacts,
}

impl pull::Pullable for Manifest {
    async fn pull(locator: String) -> Result<Self, pull::PullError> {
        let locations = locator::manifest_locations(locator);

		for location in locations {
			let response = reqwest::get(location).await;

			if let Ok(file_content) = response {
				let file_str = file_content.text().await?;
				let manifest = toml::from_str(&file_str)?;
				return Ok(manifest);
			}
		}

		return Err(pull::PullError::NotFoundError);
    }

    async fn pull_or_default(locator: Option<String>) -> Result<Self, pull::PullError> {
        if let Some(loc) = locator {
			return Self::pull(loc).await;
		} else {
			return Self::pull("humble_nogpu".to_owned()).await;
		}
    }
}
