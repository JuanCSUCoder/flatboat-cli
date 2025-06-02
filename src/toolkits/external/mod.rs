mod jinja;
mod rocker;

mod public {
  use super::*;

  pub mod jinja_tk {
    use super::*;

    pub use jinja::{
      process_template,
      TemplatingError
    };
  }

  pub mod rocker_tk {
    use super::*;
    pub use rocker::{
      configure_rocker,
      RockerConfigError,
    };
  }
}

pub use public::{
  jinja_tk,
  rocker_tk,
};