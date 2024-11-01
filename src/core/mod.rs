mod core_args;
mod core_output;
mod core_runner;

mod public {
  use super::*;

  pub mod args {
    use super::*;

    pub use core_args::{
      Cli,
      Commands,
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
}

pub use public::{
  args,
  output,
  runner,
};
