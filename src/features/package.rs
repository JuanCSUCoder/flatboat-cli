use subprocess::Exec;

use crate::args::PackageSubcommands;

/// Handles all commands related with packages
pub fn handle_pkg_cmd(pkg_cmd: PackageSubcommands) {
    match pkg_cmd {
        PackageSubcommands::Create { pkg_name } => create_pkg(&pkg_name),
        PackageSubcommands::Build { pkg_name } => build_pkg(&pkg_name),
    }
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
        let cmd = String::from("\"cd src && ros2 pkg create --build-type ament_python \"") + pkg_name;
        let res = Exec::cmd("devcontainer")
            .args(&[
                "exec", "--workspace-folder", ".", "bash", "-c", cmd.as_str()
                ])
            .join()
            .expect("Error executing command inside the container");

        if res.success() {
            // Adds Docker File Configuration
        }
    }
}

/// Builds a Docker Image for a ROS Package
fn build_pkg(pkg_name: &String) {
    // Start or check if workspace is started

    // Find Devcontainer Docker ID

    // Build Docker Image for the Package with Tag
}