use std::{fs::File, io::Read};

use result::ManifestError;

use super::pull;

mod result;

mod locator;
mod local_locator;

/// Artifacts of a Flatboat Workspace
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Artifacts {
    pub workspace: String,
    pub package: String,
    pub workload: String,
    pub bot: String,
}

/// Manifest of a Flatboat Workspace
#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub version: Option<String>,
    pub downloaded_from: Option<String>,
    pub artifacts: Artifacts,
}

impl Manifest {
    /// Gets manifest from current workspace
    pub fn new() -> Result<Self, ManifestError> {
        let locations = local_locator::get_manifest_locations()?;

        for loc in locations {
            let file_result = File::open(loc);

            if let Ok(mut manifest_file) = file_result {
                let mut content = String::new();
                manifest_file.read_to_string(&mut content).ok().ok_or(ManifestError { desc: "Unable to read manifest file, check file permissions."})?;
                let manifest: Manifest = toml::from_str(&content).ok().ok_or(ManifestError {
                    desc: "Failed manifest deserialization, make sure flatboat.toml has the correct format and syntax."
                })?;

                return Ok(manifest);
            }
        }

        return Err(ManifestError { desc: "Manifest file flatboat.toml not found" });
    }
}

impl pull::Pullable for Manifest {
    async fn pull(locator: String) -> Result<Self, pull::PullError> {
        let locations = locator::manifest_locations(locator);

        for location in locations {
            let response = reqwest::get(location.clone()).await;

            if let Ok(file_content) = response {
                let file_str = file_content.text().await?;
                let mut manifest: Manifest = toml::from_str(&file_str)?;
                manifest.downloaded_from = Some(location);
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
    use tests::pull::{PullError, Pullable};

    #[tokio::test]
    async fn pull_or_default_test() -> Result<(), PullError> {
        let man = Manifest::pull_or_default(None).await?;

        println!("{:?}", man);

        assert_eq!(man.name, "humble_nogpu".to_owned());
        assert_eq!(
            man.artifacts.workspace,
            "ghcr.io/JuanCSUCoder/flatboat-templates/roboten_ws_humble_nogpu".to_owned()
        );
        assert_eq!(
            man.artifacts.package,
            "JuanCSUCoder/flatboat-templates/pkg/humble/humble_nogpu".to_owned()
        );
        assert_eq!(
            man.artifacts.workload,
            "JuanCSUCoder/flatboat-templates/wl/humble/humble_nogpu".to_owned()
        );
        assert_eq!(
            man.artifacts.bot,
            "JuanCSUCoder/flatboat-templates/bot/humble/humble_nogpu".to_owned()
        );

        return Ok(());
    }
}
