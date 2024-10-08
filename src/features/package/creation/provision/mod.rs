mod pkg_ops;
pub mod result;

use std::{env, path::Path};

use result::ProvisionError;

use crate::{features::package::result::PackageOutput, toolkits, utils::manifest::Manifest};

/// Provisions a package with Flatboat files
pub fn provision_pkg(pkg_name: &String) -> Result<PackageOutput, ProvisionError> {
	let pkg_str_path = "./src/".to_owned() + pkg_name;
	let pkg_path = Path::new(&pkg_str_path);
		
	// Get current workspace Manifest
	let manifest = Manifest::new()?;

	// Moves inside the package direcctory
	env::set_current_dir(pkg_path)?;

	// Adds Docker File Configuration
	toolkits::oras::pull_template(&manifest.artifacts.package)?;

	// Provisions the package with Dockerfile and pkg.toml
	pkg_ops::provision_template(pkg_name)?;

	return Ok(PackageOutput {
		desc: "Successfull package creation"
	});
}
