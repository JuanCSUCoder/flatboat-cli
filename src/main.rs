mod args;
mod features;
mod toolkits;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use std::{process, io};

use args::Cli;
use clap::{Parser, CommandFactory};
use directories::ProjectDirs;

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
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

    match cli.command {
        args::Commands::Workspace(ws_args) => features::workspace::handle_ws_cmd(ws_args.subcommand),
        args::Commands::Package(pkg_args) => features::package::handle_pkg_cmd(pkg_args.subcommand),

        args::Commands::Bot(_) => todo!(),
        args::Commands::Workload(_) => todo!(),
        args::Commands::Ros2(_) => todo!(),
        args::Commands::Exec(_) => todo!(),


        args::Commands::Info => info!("FlatBoat is a command-line interface application used to access, configure and manage dockerized ROS2 development environments, and for interfacing with ros2 cli"),
        args::Commands::Generator(gen_args) => {
            let mut cmd = Cli::command_for_update();
            print_completions(gen_args.generator, &mut cmd);
        },
    }
}
