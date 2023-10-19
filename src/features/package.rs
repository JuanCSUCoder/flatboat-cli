use std::path::Path;

use crate::args::PackageSubcommands;

/// Handles all commands related with packages
pub fn handle_pkg_cmd(pkg_cmd: PackageSubcommands, path: &Path) {
    match pkg_cmd {
        PackageSubcommands::Create { pkg_name } => create_pkg(&pkg_name, path),
        PackageSubcommands::Build { pkg_name } => build_pkg(&pkg_name, path),
    }
}

/// Create a ROS Package Initialized with a Dockerfile for Building
fn create_pkg(pkg_name: &String, path: &Path) {
    // Find Devcontainer Docker ID

    // Exec Creation Command inside Devcontainer

    // Adds Docker File Configuration
}

/// Builds a Docker Image for a ROS Package
fn build_pkg(pkg_name: &String, path: &Path) {
    // Find Devcontainer Docker ID

    // Build Docker Image for the Package with Tag
}