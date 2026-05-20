use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    infile: String,
    #[arg(short, long)]
    outfile: String,
}

fn main() {
    let args = Args::parse();

    let mut infile = File::open(args.infile).unwrap();
    let mut outfile = File::create(args.outfile).unwrap();

    let mut buf = Vec::new();

    let _ = infile.read_to_end(&mut buf);
    let _ = outfile.write_all(&buf);
}
