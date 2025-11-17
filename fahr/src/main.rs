use std::io;

fn main() {
    println!("Type a temp in Celsius:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let celsius: f64 = input.trim().parse().expect("CONVERSION INPUT FAILURE");
    let fahr: f64 = celsius * 9.0 / 5.0 + 32.0;

    println!("{celsius}°C = {fahr}°F");
}
