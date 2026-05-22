use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    infile: PathBuf,
    #[arg(short, long)]
    outfile: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut infile = File::open(&args.infile).unwrap();
    let mut outfile = File::create(&args.outfile).unwrap();

    println!(
        "File: {}",
        &args.infile.file_stem().unwrap().to_string_lossy()
    );
    println!(
        "Extension: {}",
        &args.infile.extension().unwrap().to_string_lossy()
    );
    println!(
        "Filename {}.{}",
        &args.infile.file_stem().unwrap().to_string_lossy(),
        &args.infile.extension().unwrap().to_string_lossy()
    );

    let mut buf = Vec::new();

    let _ = infile.read_to_end(&mut buf);
    let _ = outfile.write_all(&buf);
}
