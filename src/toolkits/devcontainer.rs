use std::time::Duration;

use subprocess::{Exec, ExitStatus, PopenError};

use crate::utils::exec::wrapped_exec;

/// Checks if a valid devcontainer is already running, and starts it if its not
pub fn run_devcontainer() -> Result<ExitStatus, PopenError> {
    Exec::cmd("devcontainer")
        .args(&["up", "--workspace-folder", "."])
        .join()
}

/// Executes a Linux Command Inside of the Devcontainer
pub fn exec_in_shell(cmd: String, wrapped: bool) -> Result<ExitStatus, PopenError> {
    let exec = Exec::cmd("devcontainer")
        .args(&[
            "exec", "--workspace-folder", ".", "bash", "-c", cmd.as_str()
            ]);
    if wrapped {
        return wrapped_exec(exec, Some(Duration::from_secs(10)), "Shell Program");
    } else {
        return exec.join();
    }
}

/// Downloads the files from the Workspace Template
pub fn create_ws_files(image_url: &String) -> Result<ExitStatus, PopenError>{
    let exec = Exec::cmd("devcontainer")
        .args(&[
            "templates",
            "apply",
            "-t",
            &image_url,
        ]);
    return wrapped_exec(exec, Some(Duration::from_secs(10)), "Devcontainer CLI");
}
