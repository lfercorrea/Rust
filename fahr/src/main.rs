use std::io;

fn main() {
    let msg: &str = "Type a temperature in °C to convert to °F: ";
    println!("{msg}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed obtaining number from stdin");

    let celsius: f64 = input.trim().parse::<f64>().expect("Conversion failure");
    let fahr: f64 = celsius * 9.0 / 5.0 + 32.0;
    println!("{celsius}°C = {fahr}°F");
}
