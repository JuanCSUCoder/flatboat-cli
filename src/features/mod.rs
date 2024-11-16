pub mod workspace;
pub mod package;
pub mod cmds;

mod wl_feature;

pub mod public {
  use super::*;

  pub mod workload {
    use super::*;

    pub use wl_feature::{
      handle_wl_cmd,
      WorkloadCmdError,
    };
  }
}

pub use public::workload;
