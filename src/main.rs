use clap::{Parser, Subcommand};

/// Commands
#[derive(Subcommand)]
enum Commands {
    Workspace,
    Info
}

/// FlatBoat - CLI for robotics containerized docker environment
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Args::parse();
    println!("Hello, world!");
}
