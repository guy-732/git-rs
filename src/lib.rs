use std::{process::ExitCode, str::FromStr};

use clap::Parser;

mod command;

#[derive(Debug, Parser)]
#[command(version, about = "git cmd tool in rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: command::Command,
}

impl Cli {
    pub fn execute(self) -> ExitCode {
        self.command.execute()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blob => "blob",
                Self::Tree => "tree",
                Self::Commit => "commit",
                Self::Tag => "tag",
            }
        )
    }
}

impl FromStr for ObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blob" => Ok(Self::Blob),
            "tree" => Ok(Self::Tree),
            "commit" => Ok(Self::Commit),
            "tag" => Ok(Self::Tag),
            other => Err(format!("object type {:?} is not recognized", other)),
        }
    }
}
