use std::{fs::{File, Metadata}, io::{stdin, stdout, Read, Write}, path::PathBuf, time::{Duration, SystemTime}};

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

        let mut data = [0u8; 2];
        
        for (dir, _) in files {
            data.fill(0);
            input.read(&mut data)?;
            
            set(&File::open(dir.path())?, data)?;
        }
        
        let mut provided = 0;
        while let Some(read) = input.read(&mut [0; 1000]).ok().filter(|&c| c > 0) {
            provided += read;
        }

        if provided > 0 {
            eprintln!("Failed to write last {provided} byte(s): not enough files");
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

/// Gets the two bytes from a file's metadata, reading from some of its last
/// modified date.
fn get(metadata: Metadata)-> Result<[u8; 2]> {
    let nanos = metadata.modified()?
        .duration_since(SystemTime::UNIX_EPOCH)?
        .subsec_nanos();

    let data = ((nanos >> 7) & 0xffff) as u16;

    Ok(data.to_le_bytes())
}

/// Sets the two bytes in a file's metadata, modifying some of its last
/// modified date.
fn set(file: &File, data: [u8; 2]) -> Result<()> {
    let time = file.metadata()?.modified()?;
    
    // Read the nanosecond part of the time
    let nanos = time.duration_since(SystemTime::UNIX_EPOCH)?.subsec_nanos() as u64;

    // Clear the 16 destination bits
    let new_nanos = nanos & !(0xffff << 7);

    // Add them back from our trusted source
    let new_nanos = new_nanos | (u16::from_le_bytes(data) as u64) << 7;
    
    // Subtract old nanos and add new ones
    let new_time = time - Duration::from_nanos(nanos) + Duration::from_nanos(new_nanos);

    file.set_modified(new_time)?;

    Ok(())
}
