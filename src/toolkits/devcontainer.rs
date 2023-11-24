use subprocess::{Exec, PopenError, ExitStatus};

/// Executes a Linux Command Inside of the Devcontainer
pub fn exec_in_shell(cmd: String) -> Result<ExitStatus, PopenError> {
    Exec::cmd("devcontainer")
        .args(&[
            "exec", "--workspace-folder", ".", "bash", "-c", cmd.as_str()
            ])
        .join()
}
