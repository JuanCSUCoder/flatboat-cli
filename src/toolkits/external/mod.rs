mod jinja;

mod public {
  use super::*;

  pub mod jinja_tk {
    use super::*;

    pub use jinja::{
      process_template,
      TemplatingError
    };
  }
}

pub use public::jinja_tk;