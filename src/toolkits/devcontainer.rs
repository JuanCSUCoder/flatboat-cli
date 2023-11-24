use subprocess::{Exec, PopenError, ExitStatus};

/// Checks if a valid devcontainer is already running, and starts it if its not
pub fn run_devcontainer() -> Result<ExitStatus, PopenError> {
    Exec::cmd("devcontainer")
        .args(&["up", "--workspace-folder", "."])
        .join()
}

/// Executes a Linux Command Inside of the Devcontainer
pub fn exec_in_shell(cmd: String) -> Result<ExitStatus, PopenError> {
    Exec::cmd("devcontainer")
        .args(&[
            "exec", "--workspace-folder", ".", "bash", "-c", cmd.as_str()
            ])
        .join()
}
