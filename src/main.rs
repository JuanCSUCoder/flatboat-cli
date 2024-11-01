mod core;
mod features;
mod toolkits;
mod utils;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::{env, path::PathBuf, process};

use core::args;
use core::args::Cli;
use clap::Parser;
use directories::ProjectDirs;
use core::output::{ProgramError, ProgramErrorKind, ProgramOutputKind, ProgramResult};
use utils::manifest::Manifest;

async fn run_command(cli: Cli, _project_dirs: ProjectDirs) -> ProgramResult {
    let manifest = Manifest::new();

    match &manifest {
        Ok(m) => match &m.ws_path {
            Some(str_path) => {
                debug!("Found workspace and manifest at {}", &str_path);
                env::set_current_dir(PathBuf::from(str_path)).ok().ok_or(ProgramError {
                    desc: "Unable go to inside workspace",
                    kind: ProgramErrorKind::UnknownError,
                })?;
            },
            None => warn!("Workspace path not set in flatboat.toml!"),
        }
        Err(_) => warn!("Workspace commands disabled! You are not inside a workspace."),
    }

    return core::runner::handle_command(cli.command).await;
}

#[tokio::main]
async fn main() {
    core::helpers::setup_logging();
    let cli = Cli::parse();
    let project_dirs = core::helpers::setup_directories();

    let res = run_command(cli, project_dirs).await;

    if let Ok(output) = res {
        if ! matches!(output.kind, ProgramOutputKind::NoOutput) {
            info!("Finished successfully!");
            core::helpers::output_serialized(&output);
        }
    } else if let Err(err) = res {
        error!("Error: {}", err.kind);
        core::helpers::output_serialized(&err);
    }
}
