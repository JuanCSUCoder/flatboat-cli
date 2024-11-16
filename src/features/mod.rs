pub mod workspace;
pub mod package;
pub mod cmds;

mod bot_feature;

pub mod public {
  use super::*;

  pub mod bot {
    use super::*;

    pub use bot_feature::{
      handle_bot_cmd,
      BotCmdError,
    };
  }
}

pub use public::bot;
