use anyhow::{Result};
use clap::{Parser};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    n_limit: u64,
}

struct Primes(Vec<u64>);

fn main() -> Result<()> {
    let opts = Opts::parse();
    let primes = erathostenes(opts.n_limit);
    println!("Primes: {:?}", primes.0);

    Ok(())
}

fn erathostenes(n: u64) -> Primes {
    let mut _bools: Vec<bool> = vec![true; n as usize];
    _bools[0] = false;
    _bools[1] = false;
    
    for i in 2..((n as f64).sqrt() as usize) {
        if _bools[i] {
            for j in ((i * i)..(n as usize)).step_by(i) {
                _bools[j] = false;
            }
        }
    }

    let mut primes = Vec::new();

    for (key, val) in _bools.iter().enumerate() {
        if *val {
            primes.push(key as u64);
        }
    }

    Primes(primes)
}