use std::{fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf};

use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;

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
    /// Provide content of repository object
    CatFile {
        /// The hash of the object to inspect
        object: String,
        /// Pretty-print the contents of <object> based on its type.
        #[arg(short)]
        pretty_print: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    // eprintln!("{:#?}", cli);
    match cli.command {
        Command::Init => match init() {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to initialize git repository: {}", e),
        },
        Command::CatFile { object: hash, pretty_print } => match cat_file(hash, pretty_print) {
            Ok(_) => (),
            Err(e) => eprintln!("cat-file failed: {}", e),
        }
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

fn cat_file(hash: String, pretty_print: bool) -> anyhow::Result<()> {
    let mut file_path = PathBuf::from(".git/objects");
    file_path.push(&hash[..2]);
    file_path.push(&hash[2..]);
    let mut file = BufReader::new(ZlibDecoder::new(File::open(file_path)?));
    // std::io::copy(&mut file, &mut &std::io::stdout())?;
    let mut buffer = vec![];
    file.read_until(b' ', &mut buffer)?;
    let obj_type = std::str::from_utf8(&buffer)?.trim_end().to_owned();

    buffer.clear();
    file.read_until(0, &mut buffer)?;
    let obj_size = std::str::from_utf8(&buffer)?.trim_end_matches('\0').parse::<u64>()?;

    eprintln!("Object type: {}", obj_type);
    eprintln!("Object size: {}", obj_size);

    if pretty_print {
        std::io::copy(&mut file, &mut &std::io::stdout())?;
    }

    Ok(())
}
