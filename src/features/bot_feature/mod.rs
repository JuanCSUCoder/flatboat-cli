mod create_bot;

use create_bot::CreateBotError;
use thiserror::Error;

use crate::core::{args::BotSubcommands, output::{ProgramError, ProgramOutput}};

#[derive(Error, Debug)]
pub enum BotCmdError {
  #[error(transparent)]
  BotCreationError(#[from] CreateBotError),
}

/// Handles bot commands
pub fn handle_bot_cmd(bot_command: BotSubcommands) -> Result<ProgramOutput, ProgramError> {
  let result = handle_bot_result(bot_command);

  match result {
    Ok(_) => Ok(ProgramOutput {
      desc: "Bot created successfully!",
      kind: crate::core::output::ProgramOutputKind::Ok
    }),
    Err(e) => Err(ProgramError {
      desc: "Unable to process bot command!",
      kind: crate::core::output::ProgramErrorKind::BOT(e),
    }),
  }

}

fn handle_bot_result(bot_command: BotSubcommands) -> Result<(), BotCmdError> {
  match bot_command {
    BotSubcommands::Create { bot_name } => Ok(create_bot::create_bot(&bot_name)?),
    BotSubcommands::BringUp { bot_name: _ } => todo!(),
    BotSubcommands::BringDown { bot_name: _ } => todo!(),
    BotSubcommands::Refresh { bot_name: _ } => todo!(),
  }
}
