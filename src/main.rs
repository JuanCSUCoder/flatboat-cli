mod args;
mod features;
mod toolkits;
mod utils;
mod output;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::{io, process};

use args::Cli;
use clap::{Parser, CommandFactory};
use directories::ProjectDirs;
use output::{ProgramOutput, ProgramResult};

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

async fn run_command(cli: Cli, _project_dirs: ProjectDirs) -> ProgramResult {
    match cli.command {
        args::Commands::Workspace(ws_args) => features::workspace::handle_ws_cmd(ws_args.subcommand).await,
        args::Commands::Package(pkg_args) => features::package::handle_pkg_cmd(pkg_args.subcommand),

        args::Commands::Bot(_) => todo!(),
        args::Commands::Workload(_) => todo!(),
        args::Commands::Ros2(ros2_args) => features::cmds::handle_ros2_cmd(ros2_args),
        args::Commands::Exec(exec_args) => features::cmds::handle_exec_cmd(exec_args),


        args::Commands::Info => {
            info!("FlatBoat is a command-line interface application used to access, configure and manage dockerized ROS2 development environments, and for interfacing with ros2 cli");
            Ok(ProgramOutput::Ok)
        },
        args::Commands::Completion(gen_args) => {
            let mut cmd = Cli::command_for_update();
            print_completions(gen_args.shell, &mut cmd);
            Ok(ProgramOutput::NoOutput)
        },
    }
}

fn output_serialized(output: &impl serde::Serialize) {
    print!("{}", toml::ser::to_string_pretty(&output).expect("TOML Serializer"));
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
        if ! matches!(output, ProgramOutput::NoOutput) {
            output_serialized(&output);
        }
    } else if let Err(err) = res {
        output_serialized(&err);
    }
}
