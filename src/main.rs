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

    return core::runner::handle_command(cli.command);
}

fn output_serialized(output: &impl serde::Serialize) {
    info!("====== RESULT =======");
    info!("\n{}", toml::ser::to_string_pretty(&output).expect("TOML Serializer"));
    info!("=====================");
    println!("{}", serde_json::to_string(&output).expect("JSON Serializer"));
}

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    let cli = Cli::parse();

    let project_dirs = ProjectDirs::from("codes", "juancsu", "flatboat");

    if project_dirs.is_none() {
        error!("Unable to locate application folders");
        process::exit(1);
    } else {
        debug!("Located folders {:?}", &project_dirs);
    }

    let project_dirs = project_dirs.unwrap();
    let data_dir = project_dirs.data_dir();

    debug!("Data Directory: {:?}", data_dir);

    let res = run_command(cli, project_dirs).await;

    if let Ok(output) = res {
        if ! matches!(output.kind, ProgramOutputKind::NoOutput) {
            output_serialized(&output);
        }
    } else if let Err(err) = res {
        error!("Error: {}", err.kind);
        output_serialized(&err);
    }
}
