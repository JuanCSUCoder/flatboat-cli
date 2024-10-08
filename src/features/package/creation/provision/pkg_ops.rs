use std::{error::Error, fs};

use crate::utils::package_config::PackageConfig;

use super::result::UpdatePackageConfigError;

const PKG_CONFIG_COMMENTS: &'static str = r#"
# command_file = "move_run.py"
# extra_args = '"--left", "30"'
"#;

pub fn provision_template(pkg_name: &String) -> Result<(), ()> {
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
}

fn update_package_config(pkg_name: &String) -> Result<(), UpdatePackageConfigError> {
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
