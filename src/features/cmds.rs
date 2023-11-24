use crate::{args::{Ros2Args, ExecArgs}, toolkits::devcontainer};

/// Handles ROS2 Command Arguments
pub fn handle_ros2_cmd(ros2_args: Ros2Args) {
    if devcontainer::run_devcontainer().is_ok() {
        if devcontainer::exec_in_shell("ros2 ".to_string() + &ros2_args.ros2_args).is_ok() {
            debug!("Command Executed Successfully")
        } else {
            error!("Command Execution Failed")
        }
    } else {
        error!("Failed to start the devcontainer, check if the current directory is based on a valid devcontainer template")
    }
}

/// Handles Exec Command Arguments
pub fn handle_exec_cmd(exec_args: ExecArgs) {
    if devcontainer::run_devcontainer().is_ok() {
        if devcontainer::exec_in_shell(exec_args.exec_cmd).is_ok() {
            debug!("Command Executed Successfully")
        } else {
            error!("Command Execution Failed")
        }
    } else {
        error!("Failed to start the devcontainer, check if the current directory is based on a valid devcontainer template")
    }
}
