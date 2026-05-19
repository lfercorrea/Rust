use erathostenes::Primes;

fn main() {
    let primes = Primes::erathostenes(1024);
    println!("{primes}");
}
