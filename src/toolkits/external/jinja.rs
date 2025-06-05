use std::{fs, io, path::Path};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TemplatingError {
  #[error("Unable to read/write template information.")]
  IOError(#[from] io::Error),

  #[error("Error processing template.")]
  ProcesingError(#[from] minijinja::Error),
}

/// Processes template from one file and writes the result on destination
pub fn process_template<S: Serialize>(source: &Path, destination: &Path, data: &S) -> Result<(), TemplatingError> {
  let template_file = fs::read_to_string(source)?;

	let mut env = minijinja::Environment::new();
	env.add_template("dockerfile", &template_file)?;

	let template = env.get_template("dockerfile")?;

	let generated_dockerfile = template.render(data)?;

	fs::write(destination, generated_dockerfile)?;

	return Ok(());
}
