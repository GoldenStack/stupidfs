use std::{fs::{File, Metadata}, io::{stdin, stdout, Read, Write}, mem::transmute, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {

    /// The path of the root directory in which files will be stored
    #[clap(default_value = ".")]
    pub path: PathBuf,

    /// Writes all input to the root directory
    #[clap(short, group = "mode")]
    pub input: bool,

    /// Outputs all data stored in the root directory
    #[clap(short, group = "mode", default_value = "true")]
    pub output: bool,

}


fn main() -> Result<()> {
    let args = Args::parse();


    let files = WalkDir::new(args.path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|dir| dir.metadata().ok().filter(Metadata::is_file).map(|meta| (dir, meta)));
 
    if args.input {
        let mut input = stdin().lock();

        let mut data = [0u8; 3];
        
        for (dir, _) in files {
            input.read(&mut data)?;

            set(&File::open(dir.path())?, data)?;
        }

    } else if args.output {
        let mut out = stdout().lock();
        
        for (_, meta) in files {
            let data = get(meta)?;
            out.write(&data)?;
        }

        out.flush()?;
    } else {
        // Unreachable because flags cannot be disabled,
        // and output is true by default.
        unreachable!();
    }


    Ok(())
}

/// Gets the three bytes from a file's metadata, reading from some of its last
/// modified date.
fn get(metadata: Metadata)-> Result<[u8; 3]> {
    let time = get_all(metadata)?;

    Ok(time[8..11].try_into()?)
}

/// Sets the three bytes in a file's metadata, modifying some of its last
/// modified date.
fn set(file: &File, data: [u8; 3]) -> Result<()> {
    let mut time = get_all(file.metadata()?)?;

    time[8..11].copy_from_slice(&data);

    // Safety: I don't care if this breaks
    let new_time = unsafe { transmute(time) };

    file.set_modified(new_time)?;

    Ok(())
}

/// Gets the raw bytes of the [SystemTime] for the last modified date of a file.
fn get_all(metadata: Metadata) -> Result<[u8; 16]> {
    let last = metadata.modified()?;

    // Safety: I don't care if this breaks
    let time: [u8; 16] = unsafe { transmute(last) };

    Ok(time)
}