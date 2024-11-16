mod create_wl;

use create_wl::CreateWorkloadError;
use thiserror::Error;

use crate::core::{args::WorkloadSubcommands, output::{ProgramError, ProgramOutput}};

#[derive(Error, Debug)]
pub enum WorkloadCmdError {
  #[error(transparent)]
  BotCreationError(#[from] CreateWorkloadError),
}

/// Handles workload commands
pub fn handle_wl_cmd(wl_command: WorkloadSubcommands) -> Result<ProgramOutput, ProgramError> {
  let result = handle_wl_result(wl_command);

  match result {
    Ok(_) => Ok(ProgramOutput {
      desc: "Bot created successfully!",
      kind: crate::core::output::ProgramOutputKind::Ok
    }),
    Err(e) => Err(ProgramError {
      desc: "Unable to process bot command!",
      kind: crate::core::output::ProgramErrorKind::WL(e),
    }),
  }

}

fn handle_wl_result(bot_command: WorkloadSubcommands) -> Result<(), WorkloadCmdError> {
  match bot_command {
    WorkloadSubcommands::Create { wl_name } => Ok(create_wl::create_wl(&wl_name)?),
    WorkloadSubcommands::Deploy { wl_name: _ } => todo!(),
  }
}
