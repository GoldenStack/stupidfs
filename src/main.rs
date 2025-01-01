use std::{fs::File, mem::transmute, path::PathBuf};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {

    /// The path of the root directory in which files will be stored.
    #[clap(default_value = ".")]
    pub path: PathBuf,

}


fn main() -> Result<()> {
    let args = Args::parse();

    println!("CLAP args: {args:?}");


    let v = File::open(&args.path)?;
 
    println!("BEFORE: {:?}", get(&v));

    let data = [0b11111111, 0b11111111, 0b11111111];
    set(&v, data)?;

    println!("AFTER:  {:?}", get(&v));
    
    Ok(())
}

/// Gets the three bytes from a file's metadata, reading from some of its last
/// modified date.
fn get(file: &File) -> Result<[u8; 3]> {
    let time = get_all(file)?;

    Ok(time[8..11].try_into()?)
}

/// Sets the three bytes in a file's metadata, modifying some of its last
/// modified date.
fn set(file: &File, data: [u8; 3]) -> Result<()> {
    let mut time = get_all(file)?;

    time[8..11].copy_from_slice(&data);

    // Safety: I don't care if this breaks
    let new_time = unsafe { transmute(time) };

    file.set_modified(new_time)?;

    Ok(())
}

/// Gets the raw bytes of the [SystemTime] for the last modified date of a file.
fn get_all(file: &File) -> Result<[u8; 16]> {
    let last = file.metadata()?.modified()?;

    // Safety: I don't care if this breaks
    let time: [u8; 16] = unsafe { transmute(last) };

    Ok(time)
}