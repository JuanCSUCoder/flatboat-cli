use super::pull;

mod  locator;

/// Artifacts of a Flatboat Workspace
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct Artifacts {
	pub workspace: String,
	pub package: String,
	pub workload: String,
	pub bot: String,
}

/// Manifest of a Flatboat Workspace
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Manifest {
	pub name: String,
	pub downloaded_from: Option<String>,
	pub artifacts: Artifacts,
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

#[cfg(test)]
mod tests {
	use super::*;
	use tests::pull::{Pullable, PullError};

	#[tokio::test]
	async fn pull_or_default_test() -> Result<(), PullError> {
		let man = Manifest::pull_or_default(None).await?;

		println!("{:?}", man);

		assert_eq!(man.name, "humble_nogpu".to_owned());
		assert_eq!(man.artifacts.workspace, "JuanCSUCoder/flatboat-templates/roboten_ws_humble_nogpu".to_owned());
		assert_eq!(man.artifacts.package, "JuanCSUCoder/flatboat-templates/pkg/humble/humble_nogpu".to_owned());
		assert_eq!(man.artifacts.workload, "JuanCSUCoder/flatboat-templates/wl/humble/humble_nogpu".to_owned());
		assert_eq!(man.artifacts.bot, "JuanCSUCoder/flatboat-templates/bot/humble/humble_nogpu".to_owned());

		return Ok(())
	}
}
