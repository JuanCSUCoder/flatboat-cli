use clap::{error::ErrorKind, CommandFactory};

use crate::{args::{Cli, ExecArgs, Ros2Args}, toolkits::devcontainer};

fn extract_args(args: &Vec<String>) -> String {
    if args.is_empty() {
        Cli::command()
            .error(ErrorKind::MissingRequiredArgument, "No command provided! You should at least provide one argument for the command.")
            .exit();
    }

    let mut str_args = String::new();
    for arg in args {
        str_args.push_str(&arg);
        str_args.push(' ');
    }

    return str_args;
}

/// Handles ROS2 Command Arguments
pub fn handle_ros2_cmd(ros2_args: Ros2Args) {
    let args = extract_args(&ros2_args.ros2_args);

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
    let cmd = extract_args(&exec_args.exec_cmd);

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
