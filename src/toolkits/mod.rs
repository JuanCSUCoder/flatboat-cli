mod devcontainer_tk;
mod oras_tk;
mod docker_tk;
mod jinja_tk;

mod public {
  use super::*;

  pub mod devcontainer {
    pub use crate::toolkits::devcontainer_tk::{
      run_devcontainer,
      exec_in_shell,
      create_ws_files,
    };
  }

  pub mod oras {
    pub use crate::toolkits::oras_tk::pull_template;
  }

  pub mod docker {
    pub use crate::toolkits::docker_tk::build_image;
  }

  pub mod jinja {
    use super::*;

    pub use jinja_tk::{
      process_template,
      TemplatingError,
    };
  }
}

pub use public::{
  devcontainer,
  oras,
  docker,
  jinja,
};