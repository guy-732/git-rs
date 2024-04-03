use std::process::ExitCode;

use clap::Parser;
use git_starter_rust::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    // eprintln!("{:#?}", cli);
    cli.execute()
}
