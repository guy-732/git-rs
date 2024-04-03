use std::{process::ExitCode, str::FromStr};

use clap::Parser;

mod command;
use command::Command;

#[derive(Debug, Parser)]
#[command(version, about = "git cmd tool in rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn execute(self) -> ExitCode {
        self.command.execute()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ObjectType {
    Blob,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blob => "blob",
            }
        )
    }
}

impl FromStr for ObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blob" => Ok(Self::Blob),
            other => Err(format!("object type {:?} is not recognized", other)),
        }
    }
}
