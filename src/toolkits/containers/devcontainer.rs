use thiserror::Error;
use std::time::Duration;

use subprocess::{Exec, ExitStatus, PopenError};

use crate::{toolkits::{external::rocker_tk}, utils::exec::wrapped_exec};

/// Checks if a valid devcontainer is already running, and starts it if its not
pub fn run_devcontainer() -> Result<ExitStatus, PopenError> {
    let exec = Exec::cmd("devcontainer")
        .args(&["up", "--workspace-folder", "."]);

    return wrapped_exec(exec, Some(Duration::from_secs(10)), "Devcontainer CLI");
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

#[derive(Error, Debug)]
pub enum DevcontainerInitializationError {
    #[error("Failed to apply devcontainer template: {0}")]
    TemplateApplyError(#[from] PopenError),
    #[error("Failed to configure Rocker: {0}")]
    RockerError(#[from] rocker_tk::RockerConfigError),
}

/// Downloads the files from the Workspace Template
pub async fn create_ws_files(image_url: &String) -> Result<ExitStatus, DevcontainerInitializationError>{
    let exec = Exec::cmd("devcontainer")
        .args(&[
            "templates",
            "apply",
            "-t",
            &image_url,
        ]);
    let devcont_status = wrapped_exec(exec, Some(Duration::from_secs(10)), "Devcontainer CLI")?;

    if devcont_status.success() {
        rocker_tk::configure_rocker().await?;

        return Ok(ExitStatus::Exited(0));
    } else {
        error!("Failed to apply devcontainer template from {}: {:?}", image_url, devcont_status);
        return Ok(devcont_status);
    }
}
