mod args;

use args::Cli;
use clap::Parser;

fn main() {
    let _cli = Cli::parse();
    println!("Hello, world!");
}
