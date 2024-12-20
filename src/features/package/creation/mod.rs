pub mod provision;

use crate::toolkits::devcontainer;

use super::result::{PackageError, PackageOutput, PackageResult};

pub fn create_package(pkg_name: &String) -> PackageResult {
  create_ros_package(pkg_name)?;
	
	let output = provision::provision_pkg(pkg_name)?;

  Ok(output)
}

/// Creates a ROS2 package
fn create_ros_package(pkg_name: &String) -> PackageResult {
	// Exec Creation Command inside Devcontainer
	let cmd = String::from("cd src && ros2 pkg create --build-type ament_python ") + pkg_name;
	let res =
		devcontainer::exec_in_shell(cmd, true)?;
	
	if res.success() {
		return Ok(PackageOutput { desc: "ROS2 package created" });
	} else {
		return Err(PackageError::PackageCreationError);
	}
}
