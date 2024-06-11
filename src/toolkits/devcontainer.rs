use std::io::{self, BufRead, BufReader};

use subprocess::{Exec, ExitStatus, PopenError, Redirection};

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

fn wrapped_exec(exec: Exec) -> Result<ExitStatus, PopenError> {
    let exec = exec.stdout(Redirection::Pipe)
        .stderr(Redirection::Merge);
    let mut p = exec.popen()?;

    if let Some(stream) = &p.stdout {
        let reader = BufReader::new(stream);

        for line in reader.lines() {
            info!("{}", line?);
        }

        let exit = p.wait()?;

        return Ok(exit);
    } else {
        error!("Unable to get stream for devcontainer CLI");
        return Err(PopenError::IoError(io::Error::new(io::ErrorKind::Other, "Unable to get stream for devcontainer CLI")));
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
    return  wrapped_exec(exec);
}
