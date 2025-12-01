use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("Type a temp in Celsius to be converted to Fahrenheight: ");
        match io::stdout().flush() {
            Ok(t) => t,
            Err(e) => {
                println!("Error printing stdout: {e}");
            }
        }
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        let celsius: f64 = match input.trim().parse() {
            Ok(v) => v,
            Err(e) => {
                println!("Error converting from std to f64: {e}");
                continue;
            }
        };
        let fahr: f64 = celsius * 9.0 / 5.0 + 32.0;
        println!("{}°C = {}°F", celsius, fahr);
    }
}
