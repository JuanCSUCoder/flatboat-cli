use std::{env, path::Path};

use crate::{toolkits::{self, devcontainer}, utils::manifest::Manifest};

use super::result::{PackageError, PackageErrorType, PackageOutput, PackageResult};

/// Creates a ROS2 package
pub fn create_ros_package(pkg_name: &String) -> PackageResult {
	// Exec Creation Command inside Devcontainer
	let cmd = String::from("cd src && ros2 pkg create --build-type ament_python ") + pkg_name;
	let res =
		devcontainer::exec_in_shell(cmd).ok().ok_or(PackageError {
			kind: PackageErrorType::PackageCreationError,
			desc: "Unable to create ROS package. Command execution failed.",
		})?;
	
	if res.success() {
		return Ok(PackageOutput { desc: "ROS2 package created" });
	} else {
		return Err(PackageError { 
			kind: PackageErrorType::PackageCreationError, 
			desc: "Unable to create ROS2 package" 
		});
	}
}

/// Provisions a package with Flatboat files
pub fn provision_pkg(pkg_name: &String) -> PackageResult {
	let pkg_str_path = "./src/".to_owned() + pkg_name;
		let pkg_path = Path::new(&pkg_str_path);
		
	// Get current workspace Manifest
	let manifest = Manifest::new().ok().ok_or(PackageError {
		kind: PackageErrorType::ManifestNotFound,
		desc: "Unable to find manifest file, please make sure you are in the correct folder"
	})?;

	// Moves inside the package direcctory
	env::set_current_dir(pkg_path).ok().ok_or(PackageError {
		kind: PackageErrorType::PackageCreationError,
		desc: "Unable to open package folder"
	})?;

	// Adds Docker File Configuration
	toolkits::oras::pull_template(&manifest.artifacts.package).ok().ok_or(PackageError {
		kind: PackageErrorType::PackageCreationError,
		desc: "ORAS command failed, unable to pull template from registry"
	})?;

	// TODO: Apply dockerfile template

	return Ok(PackageOutput {
		desc: "Successfull package creation"
	});
}