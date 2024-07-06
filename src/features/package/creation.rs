use std::{env, error::Error, fs::{self}, path::Path};

use crate::{toolkits::{self, devcontainer}, utils::{manifest::Manifest, package_config::PackageConfig}};

use super::result::{PackageError, PackageErrorType, PackageOutput, PackageResult};

/// Creates a ROS2 package
pub fn create_ros_package(pkg_name: &String) -> PackageResult {
	// Exec Creation Command inside Devcontainer
	let cmd = String::from("cd src && ros2 pkg create --build-type ament_python ") + pkg_name;
	let res =
		devcontainer::exec_in_shell(cmd, true).ok().ok_or(PackageError {
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

	// Update pkg.toml
	update_package_config(&pkg_name).ok().ok_or(PackageError {
		kind: PackageErrorType::ConfigurationError,
		desc: "Unable to properly configure pkg.toml, check if the file exists"
	})?;

	// Apply dockerfile template
	apply_dockerfile_template().ok().ok_or(PackageError {
		kind: PackageErrorType::DockerfileError,
		desc: "Unable to generate Dockerfile from package configuration"
	})?;

	return Ok(PackageOutput {
		desc: "Successfull package creation"
	});
}

const PKG_CONFIG_COMMENTS: &'static str = r#"
# command_file = "move_run.py"
# extra_args = '"--left", "30"'
"#;

fn update_package_config(pkg_name: &String) -> Result<(), Box<dyn Error>> {
	let mut pkg_config = PackageConfig::from_current_folder()?;

	pkg_config.package_name = pkg_name.clone();
	pkg_config.command_file = None;
	pkg_config.extra_args = None;

	let file_content = toml::to_string_pretty(&pkg_config)? + PKG_CONFIG_COMMENTS;
	fs::write("pkg.toml", file_content)?;

	return Ok(());
}

fn apply_dockerfile_template() -> Result<(), Box<dyn Error>>{
	let template_file = fs::read_to_string("Dockerfile.jinja")?;

	let mut env = minijinja::Environment::new();
	env.add_template("dockerfile", &template_file)?;

	let template = env.get_template("dockerfile")?;

	let pkg_config = PackageConfig::from_current_folder()?;
	let generated_dockerfile = template.render(pkg_config)?;

	fs::write("Dockerfile", generated_dockerfile)?;

	return Ok(());
}