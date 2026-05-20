use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filter: String,
    #[arg(short, long, default_value_t = 1)]
    ditter: u64,
    #[arg(short = 'i', long, default_value_t = 1)]
    depia: u64,
    #[arg(short = 'o', long, default_value_t = 1)]
    donochrome: u64,
    #[arg(short, long, default_value_t = 1)]
    manual: u64,
    #[arg(short = 'r', long, default_value_t = 1)]
    monochrome: u64,
}

fn main() {
    let args = Args::parse();

    println!("Hello, {}!", args.filter);
}
