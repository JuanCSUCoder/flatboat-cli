use std::time::Duration;

use subprocess::{Exec, PopenError};

use crate::utils;

/// Pulls a template from a container registry
pub fn pull_template(template_name: String) -> Result<subprocess::ExitStatus, PopenError> {
  let cmd = Exec::cmd("oras")
    .args(&[
      "pull",
      template_name.as_str()
    ]);
  
  return utils::exec::wrapped_exec(cmd, Some(Duration::from_secs(15)), "ORAS");
}