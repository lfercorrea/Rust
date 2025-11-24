use std::io::{self, Write};

pub fn get_char(msg: &str) -> char {
    loop {
        let mut input = String::new();

        print!("{msg}");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading stdin");
        let trimmed = input.trim();

        match trimmed.chars().next() {
            Some(c) => return c,
            None => {
                println!("Error getting the char from input");
            }
        }
    }
}

pub fn get_string(msg: &str) -> String {
    let mut input = String::new();

    print!("{msg}");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading stdin");

    input.trim_end().to_string()
}

pub fn get_i32(msg: &str) -> i32 {
    loop {
        let mut input = String::new();

        print!("{msg}");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading stdin");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                panic!("Invalid input")
            }
        }
    }
}

pub fn get_f64(msg: &str) -> f64 {
    loop {
        let mut input = String::new();

        print!("{msg}");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading stdin");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                panic!("Invalid input");
            }
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
