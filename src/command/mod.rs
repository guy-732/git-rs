use std::process::ExitCode;

use clap::Subcommand;


mod cat_file;
mod hash_object;
mod init;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize a git repository in the current directory
    Init(init::Args),
    /// Provide content of repository object
    CatFile(cat_file::Args),
    /// Compute object hash and optionally creates a blob from a file
    HashObject(hash_object::Args),
}

impl Command {
    pub fn execute(self) -> ExitCode {
        match self {
            Command::Init(args) => match init::init(args) {
                Ok(code) => code,
                Err(e) => {
                    eprintln!("Failed to initialize git repository: {}", e);
                    ExitCode::FAILURE
                }
            },
            Command::CatFile(args) => match cat_file::cat_file(args) {
                Ok(code) => code,
                Err(e) => {
                    eprintln!("cat-file failed: {}", e);
                    ExitCode::FAILURE
                }
            },
            Command::HashObject(args) => match hash_object::hash_object(args) {
                Ok(code) => code,
                Err(e) => {
                    eprintln!("hash-object failed: {}", e);
                    ExitCode::FAILURE
                }
            },
        }
    }
}
