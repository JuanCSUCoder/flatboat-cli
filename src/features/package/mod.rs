pub mod creation;
pub mod result;

mod pkg_build;

use std::path::Path;

use result::{PackageError, PackageResult};

use crate::{
    args::PackageSubcommands,
    output::{ProgramError, ProgramErrorKind, ProgramOutput, ProgramOutputKind},
    toolkits::{self, devcontainer}, utils::package_config::PackageConfig,
};

/// Handles all commands related with packages
pub fn handle_pkg_cmd(pkg_cmd: PackageSubcommands) -> Result<ProgramOutput, ProgramError> {
    let pkg_res = match pkg_cmd {
        PackageSubcommands::Create { pkg_name } => create_pkg(&pkg_name),
        PackageSubcommands::Build { pkg_name } => build_pkg(&pkg_name),
    };


    if let Ok(pkg_out) = pkg_res {
        return Ok(ProgramOutput { kind: ProgramOutputKind::PKGCreate(pkg_out), desc: "Package created successfully" });
    } else if let Err(pkg_err) = pkg_res {
        return  Err(ProgramError { kind: ProgramErrorKind::PKGCreate(pkg_err), desc: "Failed package creation" });
    } else {
        return Err(ProgramError { kind: ProgramErrorKind::UnknownError, desc: "" });
    }
}

/// Create a ROS Package Initialized with a Dockerfile for Building
fn create_pkg(pkg_name: &String) -> PackageResult {
    // Start or check if workspace is started
    let res = devcontainer::run_devcontainer().ok().ok_or(PackageError::DevcontainerError)?;

    if res.success() {
        return creation::create_package(pkg_name);
    } else {
        return Err(PackageError::DevcontainerError);
    }
}

/// Builds a Docker Image for a ROS Package
fn build_pkg(pkg_name: &str) -> PackageResult {
    // Start or check if workspace is started
    devcontainer::run_devcontainer().ok().ok_or(PackageError::DevcontainerError)?;

    // Get workspace and package paths
    let ws = Path::new(".").canonicalize()?;
    let pkg_path = ws.join("src").join(pkg_name);
    let template = pkg_path.join("Dockerfile.jinja");
    let dockerfile = pkg_path.join("Dockerfile");
    let pkg_config = PackageConfig::from_path(&pkg_path)?;

    // Generate Dockerfile from template
    toolkits::jinja::process_template(&template, &dockerfile, &pkg_config)?;

    // Build package docker image
    pkg_build::build_package(pkg_name, &ws, &dockerfile)?;

    return Err(PackageError::NotImplemented);
}
