use std::fs;

use clap::{Parser, Subcommand};

#[derive(Debug)]
#[derive(Parser)]
#[command(version, about = "git cmd tool in rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Command {
    /// Initialize a git repository in the current directory
    Init,
}

fn main() {
    let cli = Cli::parse();
    // eprintln!("{:#?}", cli);
    match cli.command {
        Command::Init => match init() {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to initialize git repository: {:?}", e),
        },
    }
}

fn init() -> anyhow::Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");

    Ok(())
}
