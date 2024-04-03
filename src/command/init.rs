use std::{fs, process::ExitCode};

#[derive(clap::Args, Debug, Clone)]
pub struct Args {}

// TODO: handle when git folder already exists
pub fn init(_args: Args) -> anyhow::Result<ExitCode> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");

    Ok(ExitCode::SUCCESS)
}
