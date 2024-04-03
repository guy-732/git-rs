use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Seek, Write},
    path::PathBuf,
    process::ExitCode,
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use hex::ToHex;
use sha1::{Digest, Sha1};

use clap::Subcommand;

use crate::ObjectType;

#[derive(Debug, Subcommand)]
pub enum Command {
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
    /// Compute object hash and optionally creates a blob from a file
    HashObject {
        /// The file to hash
        file: PathBuf,
        /// Actually write the object into the object database.
        #[arg(short)]
        write: bool,
    },
}

impl Command {
    pub fn execute(self) -> ExitCode {
        match self {
            Command::Init => match init() {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("Failed to initialize git repository: {}", e);
                    ExitCode::FAILURE
                }
            },
            Command::CatFile {
                object: hash,
                pretty_print,
            } => match cat_file(hash, pretty_print) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("cat-file failed: {}", e);
                    ExitCode::FAILURE
                }
            },
            Command::HashObject { file, write } => match hash_object(file, write) {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("hash-object failed: {}", e);
                    ExitCode::FAILURE
                }
            },
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
    // io::copy(&mut file, &mut &io::stdout())?;
    let mut buffer = vec![];
    file.read_until(b' ', &mut buffer)?;
    let _obj_type = std::str::from_utf8(&buffer)?.trim_end().to_owned();

    buffer.clear();
    file.read_until(0, &mut buffer)?;
    let _obj_size = std::str::from_utf8(&buffer)?
        .trim_end_matches('\0')
        .parse::<u64>()?;

    // eprintln!("Object type: {}", obj_type);
    // eprintln!("Object size: {}", obj_size);

    if pretty_print {
        io::copy(&mut file, &mut &io::stdout())?;
    }

    Ok(())
}

fn hash_object(file: PathBuf, write: bool) -> anyhow::Result<()> {
    let mut file = File::open(file)?;
    let object_type = ObjectType::Blob;
    let object_length = file.metadata()?.len();

    let mut hasher = Sha1::new();
    write_object_to(object_type, object_length, &mut file, &mut hasher)?;
    let object_hash: String = hasher.finalize().encode_hex();

    println!("{}", object_hash);

    if write {
        file.rewind()?;
        write_object(object_hash, object_type, object_length, &mut file)?;
    }

    Ok(())
}

// -----------------------------------------------------------

fn write_object<R: Read>(
    object_hash: String,
    object_type: ObjectType,
    object_length: u64,
    object_data: &mut R,
) -> anyhow::Result<()> {
    let mut path = PathBuf::from(".git/objects");
    path.push(&object_hash[..2]);

    if !path.exists() {
        fs::create_dir(&path)?;
    }

    path.push(&object_hash[2..]);

    let mut file = ZlibEncoder::new(File::create(path)?, Compression::default());
    write_object_to(object_type, object_length, object_data, &mut file)?;
    file.try_finish()?;

    Ok(())
}

fn write_object_to<R: Read, W: Write>(
    object_type: ObjectType,
    object_length: u64,
    object_data: &mut R,
    out: &mut W,
) -> anyhow::Result<()> {
    write!(out, "{} {}\0", object_type, object_length)?;
    io::copy(object_data, out)?;
    Ok(())
}
