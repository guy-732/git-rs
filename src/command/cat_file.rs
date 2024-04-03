use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::ExitCode,
};

use clap::ArgGroup;
use flate2::read::ZlibDecoder;

#[derive(clap::Args, Debug, Clone)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .args([
            "print_type",
            "print_size",
            "pretty_print",
        ])
))]
pub struct Args {
    /// The hash of the object to inspect
    object: String,

    /// Instead of the content, show the object type identified by <object>.
    #[arg(short = 't')]
    print_type: bool,

    /// Instead of the content, show the object size identified by <object>.
    #[arg(short = 's')]
    print_size: bool,

    /// Pretty-print the contents of <object> based on its type.
    #[arg(short = 'p')]
    pretty_print: bool,
}

pub fn cat_file(args: Args) -> anyhow::Result<ExitCode> {
    let mut file_path = PathBuf::from(".git/objects");
    file_path.push(&args.object[..2]);
    file_path.push(&args.object[2..]);
    let mut file = BufReader::new(ZlibDecoder::new(File::open(file_path)?));
    // io::copy(&mut file, &mut &io::stdout())?;
    let mut buffer = vec![];
    file.read_until(b' ', &mut buffer)?;
    let obj_type = std::str::from_utf8(&buffer)?.trim_end().to_owned();

    buffer.clear();
    file.read_until(0, &mut buffer)?;
    let obj_size = std::str::from_utf8(&buffer)?
        .trim_end_matches('\0')
        .parse::<u64>()?;

    if args.print_type {
        println!("{}", obj_type);
    } else if args.print_size {
        println!("{}", obj_size);
    } else if args.pretty_print {
        io::copy(&mut file, &mut &io::stdout())?;
    }

    Ok(ExitCode::SUCCESS)
}
