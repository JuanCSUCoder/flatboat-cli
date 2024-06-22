use std::path::PathBuf;

use super::result::ManifestError;

/// Function description
pub fn get_manifest_locations() -> Result<Vec<PathBuf>, ManifestError> {
	let mut locations: Vec<PathBuf> = vec![];

    let current_dir = std::env::current_dir().ok().ok_or(ManifestError {
        desc: "Unable to get current directory"
    })?;

	let ancestors = current_dir.ancestors();

	for ancestor in ancestors {
		let manifest_path = ancestor.join("flatboat.toml");

		locations.push(manifest_path);
	}

	return Ok(locations);
}

#[test]
fn name() {
	unimplemented!();
}