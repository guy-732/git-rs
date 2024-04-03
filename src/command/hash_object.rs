use std::{fs::{self, File}, io::{self, Read, Seek, Write}, path::PathBuf, process::ExitCode};

use flate2::{write::ZlibEncoder, Compression};
use hex::ToHex;
use sha1::{Digest, Sha1};

use crate::ObjectType;

#[derive(clap::Args, Debug, Clone)]
pub struct Args {
    /// Actually write the object into the object database.
    #[arg(short = 'w')]
    write: bool,

    /// The file to hash
    file: PathBuf,
}

pub fn hash_object(args: Args) -> anyhow::Result<ExitCode> {
    let object_type = ObjectType::Blob;
    let mut file = File::open(args.file.as_path())?;
    let object_length = file.metadata()?.len();

    let mut hasher = Sha1::new();
    write_object_to(object_type, object_length, &mut file, &mut hasher)?;
    let object_hash: String = hasher.finalize().encode_hex();

    println!("{}", object_hash);

    if args.write {
        file.rewind()?;
        write_object(object_hash, object_type, object_length, &mut file)?;
    }

    Ok(ExitCode::SUCCESS)
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
    let file = file.finish()?;
    let mut perms = file.metadata()?.permissions();
    perms.set_readonly(true);
    file.set_permissions(perms)?;

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
