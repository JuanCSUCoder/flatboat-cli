mod containers;
mod external;

mod public {
  use super::{
    containers::{
      devcontainer_tk,
      docker_tk,
      oras_tk,
    },
    external::jinja_tk
  };

  pub mod devcontainer {
    use super::*;

    pub use devcontainer_tk::{
      run_devcontainer,
      exec_in_shell,
      create_ws_files,
    };
  }

  pub mod oras {
    use super::*;

    pub use oras_tk::pull_template;
  }

  pub mod docker {
    use super::*;

    pub use docker_tk::build_image;
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