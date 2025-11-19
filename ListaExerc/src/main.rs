use std::{
    ffi::c_double,
    io::{self, Read, Write, stdout},
    result,
};

fn main() {
    // // Group I
    // // 1
    // let mut input = String::new();
    // println!("Type a value in meters:");
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Error catching input via stdin");

    // let meters: i32 = input
    //     .trim()
    //     .parse()
    //     .expect("Failed converting input to integer 'meters'");

    // let decimeters: i32 = meters * 10;
    // let centimeters: i32 = meters * 100;
    // let milimeters: i32 = meters * 1000;

    // println!(
    //     "\x1b[1;32mMeter conversion:\x1b[0m\n\
    //     Decimeters: {decimeters} dm\n\
    //     Centimeters: {centimeters} cm\n\
    //     Milimeters: {milimeters} mm"
    // );

    // // 2
    // for i in 1..11 {
    //     for j in 1..10 {
    //         let prod = i * j;
    //         print!("{j} x {i} = {prod}\t");
    //     }
    //     println!(" ");
    // }

    // //3
    // let hex = "0123456789ABCDEF".as_bytes();
    // let mut buffer = String::new();
    // let mut result = String::new();
    // let mut idx: usize;
    // let mut n: u32 = 2983;

    // while n > 0 {
    //     idx = (n % 16) as usize;
    //     buffer.push(hex[idx] as char);
    //     n /= 16;
    // }

    // for ch in buffer.chars().rev() {
    //     result.push(ch);
    // }

    // println!("{buffer}");
    // println!("{result}");

    // println!("Type a decimal number to print his hexa and octal:");
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed catching the number from stdin");

    // let n: u32 = input
    //     .trim()
    //     .parse::<u32>()
    //     .expect("Failed converting input to number");

    // print!("N: {n}\nHex: {n:x}\nOct: {n:o}\n");

    // //4
    // let msg: &str = "Type a temperature in °C to convert to °F: ";
    // println!("{msg}");
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed obtaining number from stdin");

    // let celsius: f64 = input.trim().parse::<f64>().expect("Conversion failure");
    // let fahr: f64 = celsius * 9.0 / 5.0 + 32.0;
    // println!("{celsius}°C = {fahr}°F");

    // //5
    // let mut input = String::new();
    // let mut numbers: Vec<f64> = Vec::new();

    // for i in 1..=2 {
    //     print!("Type the number {i}: ");
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Reading stdin failure");
    //     let n = input
    //         .trim()
    //         .parse::<f64>()
    //         .expect("Input conversion to u32 failure");
    //     numbers.push(n);
    // }

    // let sum: f64 = numbers[0] + numbers[1];
    // let prod: f64 = numbers[0] * (numbers[1] * numbers[1]);
    // let quad: f64 = numbers[0] * numbers[0];
    // let sqrt_sum_quad: f64 = ((numbers[0] * numbers[0]) + (numbers[1] * numbers[1])).sqrt();
    // let angle: f64 = (numbers[0] - numbers[1]).sin();
    // let module: f64 = numbers[0].abs();
    // println!(
    //     "Sum {} + {}: {sum}\n\
    //     Product {} * {}²: {prod}\n\
    //     {}² power: {quad}\n\
    //     Sqrt({}² + {}²): {sqrt_sum_quad}\n\
    //     Sin({} - {}): {angle}\n\
    //     Abs({}): {module}",
    //     numbers[0],
    //     numbers[1],
    //     numbers[0],
    //     numbers[1],
    //     numbers[0],
    //     numbers[0],
    //     numbers[1],
    //     numbers[0],
    //     numbers[1],
    //     numbers[0]
    // );

    //Group II
    // 1
    // let mut input = String::new();
    // let mut values: Vec<u32> = Vec::new();

    // for i in 1..=2 {
    //     print!("Type the value {i}: ");
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed reading stdin to input");
    //     let v = input.trim().parse().unwrap();
    //     values.push(v);
    // }

    // if values[0] < values[1] {
    //     println!("{}, {}", values[0], values[1]);
    // }
    // if values[0] > values[1] {
    //     println!("{}, {}", values[1], values[0]);
    // }
    // if values[0] == values[1] {
    //     println!("Equal values.");
    // }

    // 2
    // println!("{:<8} {:<8} {:<8}", "Dec", "Hex", "Char");
    // for i in 34u8..128 {
    //     println!("{:<8} 0x{:<4X}   {:<8}", i, i, i as char);
    // }

    // 3
    // let mut input = String::new();
    // print!("Type a price: ");
    // io::stdout().flush().unwrap();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed reading from stdin to input");
    // let mut price: f64 = input.trim().parse().unwrap();
    // price = (price * 1.1 * (price < 100.0) as u8 as f64)
    //     + (price * 1.2 * (price >= 100.0) as u8 as f64);

    // println!("Price: US$ {price:.2}");

    // 4
    // let mut input = String::new();
    // print!("Type a value: ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut input).unwrap();
    // let value1: f64 = input.trim().parse().unwrap();

    // input.clear();
    // print!("Type a operator: ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut input).unwrap();
    // let operator: char = input.trim().parse().unwrap();

    // input.clear();
    // print!("Type another value: ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut input).unwrap();
    // let value2: f64 = input.trim().parse().unwrap();
    let prompts = ["Type the value 1", "Type an operator", "Type the value 2"];
    let mut values = [0.0_f64; 2];
    let mut operator: char = ' ';
    let mut input = String::new();

    for (i, prompt) in prompts.iter().enumerate() {
        print!("{prompt}: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match i {
            0 => values[0] = input.trim().parse().unwrap(),
            1 => operator = input.trim().parse().unwrap(),
            2 => values[1] = input.trim().parse().unwrap(),
            _ => unreachable!(),
        }
    }

    let result = if operator == '+' {
        values[0] + values[1]
    } else if operator == '-' {
        values[0] - values[1]
    } else if operator == '/' && values[1] != 0.0 {
        values[0] / values[1]
    } else if operator == '*' {
        values[0] * values[1]
    } else {
        0.0
    };

    println!("{} {} {} = {}", values[0], operator, values[1], result);
}
