use std::{io::{self, BufRead, BufReader, ErrorKind}, time::Duration};

use subprocess::{Exec, PopenError, Redirection};

pub fn wrapped_exec(exec: Exec, timeout: Option<Duration>, program_name: &str) -> Result<subprocess::ExitStatus, PopenError> {
    let exec = exec.stdout(Redirection::Pipe)
        .stderr(Redirection::Merge);
    let mut p = exec.popen()?;

    if let Some(stream) = &p.stdout {
        let reader = BufReader::new(stream);

        for line in reader.lines() {
            let sanitized = line?.replace(|c: char| !c.is_ascii() || c.is_control(), "").trim().to_owned();
            if sanitized.len() > 1 {
                info!("{}\r", sanitized);
            }
        }

        let exit;
        if let Some(timeout_duration) = timeout {
            exit = p.wait_timeout(timeout_duration)?.ok_or(PopenError::IoError(io::Error::new(ErrorKind::Other, program_name.to_owned() + " command timeout")))?;
        } else {
            exit = p.wait()?;
        }

        return Ok(exit);
    } else {
        error!("Unable to get stream for {}", program_name);
        return Err(PopenError::IoError(io::Error::new(io::ErrorKind::Other, "Unable to get stream for ".to_owned() + program_name)));
    }
}