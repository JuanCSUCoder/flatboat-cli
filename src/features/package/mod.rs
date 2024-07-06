pub mod creation;
pub mod result;

use result::{PackageError, PackageErrorType, PackageResult};

use crate::{
    args::PackageSubcommands,
    output::{ProgramError, ProgramErrorKind, ProgramOutput, ProgramOutputKind},
    toolkits::devcontainer,
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
    let res = devcontainer::run_devcontainer().ok().ok_or(PackageError {
        kind: PackageErrorType::DevcontainerError,
        desc: "Unable to start current folder devcontainer. Command execution failed.",
    })?;

    if res.success() {
        creation::create_ros_package(pkg_name)?;

        return creation::provision_pkg(pkg_name);
    } else {
        return Err(PackageError {
            kind: PackageErrorType::DevcontainerError,
            desc: "Unable to start current folder devcontainer. Non zero exit status.",
        });
    }
}

/// Builds a Docker Image for a ROS Package
fn build_pkg(_pkg_name: &String) -> PackageResult {
    // Start or check if workspace is started
    devcontainer::run_devcontainer().ok().ok_or(PackageError {
        kind: PackageErrorType::DevcontainerError,
        desc: "Unable to start current folder devcontainer. Command execution failed.",
    })?;

    // Build package docker image

    return Err(PackageError { kind: PackageErrorType::NotImplemented, desc: "Package build not implemented yet!" })
}
