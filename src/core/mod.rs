mod core_args;
mod core_output;
mod core_runner;
mod core_helpers;

#[allow(unused_imports)]
mod public {
  use super::*;

  pub mod args {
    use super::*;

    pub use core_args::{
      Cli,
      Commands,

      WorkspaceSubcommands,
      BotSubcommands,
      PackageSubcommands,
      WorkloadSubcommands,

      BotArgs,
      CompletionArgs,
      ExecArgs,
      PackageArgs,
      Ros2Args,
      WorkloadArgs,
      WorkspaceArgs,
    };
  }

  pub mod output {
    use super::*;

    pub use core_output::{ProgramError, ProgramErrorKind, ProgramOutput, ProgramOutputKind, ProgramResult};
  }

  pub mod runner {
    use super::*;

    pub use core_runner::handle_command;
  }

  pub mod helpers {
    use super::*;

    pub use core_helpers::{
      setup_logging,
      setup_directories,
      output_serialized,
    };
  }
}

pub use public::{
  args,
  output,
  runner,
  helpers,
};
