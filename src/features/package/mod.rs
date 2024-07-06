pub mod result;

use std::{env, path::Path};

use result::{PackageError, PackageErrorType, PackageOutput, PackageResult};

use crate::{
    args::PackageSubcommands,
    output::{ProgramError, ProgramErrorKind, ProgramOutput, ProgramOutputKind},
    toolkits::{self, devcontainer}, utils::manifest::Manifest,
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
        // Exec Creation Command inside Devcontainer
        let cmd = String::from("cd src && ros2 pkg create --build-type ament_python ") + pkg_name;
        let res =
            devcontainer::exec_in_shell(cmd).ok().ok_or(PackageError {
                kind: PackageErrorType::PackageCreationError,
                desc: "Unable to create ROS package. Command execution failed.",
            })?;
        
        let pkg_str_path = "./src/".to_owned() + pkg_name;
        let pkg_path = Path::new(&pkg_str_path);
        
        // Get current workspace Manifest
        let manifest = Manifest::new().ok().ok_or(PackageError {
            kind: PackageErrorType::ManifestNotFound,
            desc: "Unable to find manifest file, please make sure you are in the correct folder"
        })?;

        if res.success() {
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
        } else {
            return Err(PackageError {
                kind: PackageErrorType::PackageCreationError,
                desc: "Unable to create ROS package. Non zero exit status.",
            });
        }
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
