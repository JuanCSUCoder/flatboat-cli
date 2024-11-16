use std::io;

use clap::CommandFactory;

use crate::core::args::{Cli, Commands};
use crate::core::output::{ProgramOutput, ProgramOutputKind};
use crate::features;

use super::output::ProgramResult;

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

/// Handles the command information to the appropiate handler
pub async fn handle_command(cmd: Commands) -> ProgramResult {
  match cmd {
        Commands::Workspace(ws_args) => features::workspace::handle_ws_cmd(ws_args.subcommand).await,
        Commands::Package(pkg_args) => features::package::handle_pkg_cmd(pkg_args.subcommand),

        Commands::Bot(bot_args) => features::bot::handle_bot_cmd(bot_args.subcommand),
        Commands::Workload(_) => todo!(),
        Commands::Ros2(ros2_args) => features::cmds::handle_ros2_cmd(ros2_args),
        Commands::Exec(exec_args) => features::cmds::handle_exec_cmd(exec_args),


        Commands::Info => {
            info!("FlatBoat is a command-line interface application used to access, configure and manage dockerized ROS2 development environments, and for interfacing with ros2 cli");
            Ok(ProgramOutput {kind: ProgramOutputKind::Ok, desc: "OK"})
        },
        Commands::Completion(gen_args) => {
            let mut cmd = Cli::command_for_update();
            print_completions(gen_args.shell, &mut cmd);
            Ok(ProgramOutput {kind: ProgramOutputKind::NoOutput, desc: ""})
        },
    }
}
