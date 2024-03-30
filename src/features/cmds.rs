use clap::{error::ErrorKind, CommandFactory};

use crate::{args::{Cli, ExecArgs, Ros2Args}, toolkits::devcontainer};

fn assert_args_provided(args: &Vec<String>) {
    if args.is_empty() {
        Cli::command()
            .error(ErrorKind::MissingRequiredArgument, "No command provided! You should at least provide one argument for the command.")
            .exit();
    }
}

/// Handles ROS2 Command Arguments
pub fn handle_ros2_cmd(ros2_args: Ros2Args) {
    assert_args_provided(&ros2_args.ros2_args);

    let mut args = "".to_string();
    for arg in ros2_args.ros2_args {
        args.push_str(&arg);
    }

    if devcontainer::run_devcontainer().is_ok() {
        if devcontainer::exec_in_shell("ros2 ".to_string() + &args).is_ok() {
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
    assert_args_provided(&exec_args.exec_cmd);

    let mut cmd = "".to_string();
    for cmd_part in exec_args.exec_cmd {
        cmd.push_str(&cmd_part);
    }

    if devcontainer::run_devcontainer().is_ok() {
        if devcontainer::exec_in_shell(cmd).is_ok() {
            debug!("Command Executed Successfully")
        } else {
            error!("Command Execution Failed")
        }
    } else {
        error!("Failed to start the devcontainer, check if the current directory is based on a valid devcontainer template")
    }
}
