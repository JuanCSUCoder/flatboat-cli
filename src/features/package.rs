use subprocess::Exec;

use crate::{args::PackageSubcommands, output::{ProgramError, ProgramOutput}, toolkits::devcontainer};

/// Handles all commands related with packages
pub fn handle_pkg_cmd(pkg_cmd: PackageSubcommands) -> Result<ProgramOutput, ProgramError> {
    match pkg_cmd {
        PackageSubcommands::Create { pkg_name } => create_pkg(&pkg_name),
        PackageSubcommands::Build { pkg_name } => build_pkg(&pkg_name),
    }

    return Ok(ProgramOutput::NoOutput);
}

/// Create a ROS Package Initialized with a Dockerfile for Building
fn create_pkg(pkg_name: &String) {
    // Start or check if workspace is started
    let res = Exec::cmd("devcontainer")
        .args(&["up", "--workspace-folder", "."])
        .join()
        .expect("Error Launching Devcontainer");

    if res.success() {
        // Exec Creation Command inside Devcontainer
        let cmd = String::from("cd src && ros2 pkg create --build-type ament_python ") + pkg_name;
        let res = devcontainer::exec_in_shell(cmd)
            .expect("Error executing command inside the container");

        if res.success() {
            // TODO: Adds Docker File Configuration
        }
    }
}

/// Builds a Docker Image for a ROS Package
fn build_pkg(_pkg_name: &String) {
    todo!()

    // TODO: Start or check if workspace is started

    // TODO: Find Devcontainer Docker ID

    // TODO: Build Docker Image for the Package with Tag
}