use std::path::Path;

use subprocess::Exec;

use crate::utils::exec::wrapped_exec;

/// Builds a docker image from a Dockerfile inside an specific docker context.
pub fn build_image(context_path: &Path, dockerfile_path: &Path, image_name: &str) -> Result<subprocess::ExitStatus, subprocess::PopenError> {
  let exec = Exec::cmd("docker")
    .args(&[
      "build",
      "-f",
      dockerfile_path.to_str().unwrap(),
      "-t",
      image_name,
      context_path.to_str().unwrap(),
    ]); 
  
  return wrapped_exec(exec, None, "Docker Build");
}
