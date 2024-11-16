mod manifest_utils;
mod constants_utils;
mod exec_utils;
mod package_config_utils;
mod pull_utils;
mod result_utils;

mod public {
  pub mod manifest {
    pub use crate::utils::manifest_utils::Manifest;
    pub mod result {
      pub use crate::utils::manifest_utils::result::ManifestError;
    }
  }

  pub mod pull {
    pub use crate::utils::pull_utils::{
      Pullable,
      PullError,
    };
  }

  pub mod constants {
    pub use crate::utils::constants_utils::BASE_URL;
  }

  pub mod exec {
    pub use crate::utils::exec_utils::wrapped_exec;
  }

  pub mod package_config {
    pub use crate::utils::package_config_utils::PackageConfig;
  }

  pub mod result {
    pub use crate::utils::result_utils::PackageConfigError;
  }
}

pub use public::{
  manifest,
  pull,
  constants,
  exec,
  package_config,
  result
};
