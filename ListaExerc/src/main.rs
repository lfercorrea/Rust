use std::{
    arch::x86_64::_MM_FLUSH_ZERO_MASK,
    ffi::c_double,
    fs::read_link,
    io::{self, Read, Write, stdin, stdout},
    result, string,
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
    // variation ... ->
    // let prompts = ["Type the value 1", "Type an operator", "Type the value 2"];
    // let mut values = [0.0_f64; 2];
    // let mut operator: char = ' ';
    // let mut input = String::new();

    // for (i, prompt) in prompts.iter().enumerate() {
    //     print!("{prompt}: ");
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin().read_line(&mut input).unwrap();

    //     match i {
    //         0 => values[0] = input.trim().parse().unwrap(),
    //         1 => operator = input.trim().parse().unwrap(),
    //         2 => values[1] = input.trim().parse().unwrap(),
    //         _ => unreachable!(),
    //     }
    // }

    // let result = if operator == '+' {
    //     values[0] + values[1]
    // } else if operator == '-' {
    //     values[0] - values[1]
    // } else if operator == '/' && values[1] != 0.0 {
    //     values[0] / values[1]
    // } else if operator == '*' {
    //     values[0] * values[1]
    // } else {
    //     0.0
    // };

    // println!("{} {} {} = {}", values[0], operator, values[1], result);
    // 5
    // let mut input = String::new();
    // let mut grades: Vec<f64> = Vec::new();

    // 'hall: loop {
    //     let mut avg: f64 = 0.0;
    //     for i in 1..=2 {
    //         print!("Type the course grade {}: ", i);
    //         io::stdout().flush().expect("FLUSHING TO STDOUT FAILURE");
    //         input.clear();
    //         io::stdin()
    //             .read_line(&mut input)
    //             .expect("READING STDIN FAILURE");
    //         let grade: f64 = input.trim().parse().expect("PARSING INPUT FAILURE");
    //         avg += grade;

    //         if grade == 50_f64 {
    //             break 'hall;
    //         }

    //         grades.push(grade);
    //     }

    //     avg /= 2_f64;

    //     println!("Avg: {avg:.3}");
    // }

    // group iii
    // 1 - skipped
    // 2
    // let mut numbers: Vec<i64> = Vec::new();
    // let mut input = String::new();

    // for i in 1..3 {
    //     print!("Type the number {i}: ");
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("READING STDIN FAILURE");
    //     numbers.push(input.trim().parse::<i64>().expect("PARSING INPUT FAILURE"));
    // }

    // let sum = sum(numbers[0], numbers[1]);
    // let sub = sub(numbers[0], numbers[1]);

    // println!("{} + {} = {}", numbers[0], numbers[1], sum);
    // println!("{} - {} = {}", numbers[0], numbers[1], sub);

    // fn sum(n1: i64, n2: i64) -> i64 {
    //     n1 + n2
    // }

    // fn sub(n1: i64, n2: i64) -> i64 {
    //     n1 - n2
    // }

    // group iv
    // 1
    // let base: f64 = 2_f64;
    // let expoent: i64 = 20;
    // let result = mypow(base, expoent);

    // println!("Pow {base}^{expoent} = {result}");

    // fn mypow(base: f64, expoent: i64) -> f64 {
    //     let mut sum: f64 = 1_f64;
    //     for _ in 0..expoent {
    //         sum *= base;
    //     }

    //     sum
    // }

    // 2
    // println!("{}! = {}", 5, fatorial(5));

    // fn fatorial(n: i64) -> i64 {
    //     let mut n = n;
    //     let mut a: i64 = 1;
    //     while n > 1 {
    //         a *= n;
    //         n -= 1;
    //     }

    //     a
    // }

    // 3
    // let mut input = String::new();
    // print!("Type a value for x in e^x: ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut input).unwrap();
    // let x: f64 = input.trim().parse().unwrap();
    // let mut e: f64 = 1.0;
    // let mut term: f64 = 1.0;
    // let mut i: f64 = 1.0;
    // while term > 10e-6 {
    //     term *= x / i;
    //     e += term;
    //     i += 1.0;
    // }

    // println!("e^{x} = {e}");

    // 4
    // let mut input = String::new();

    // print!("Type the expent amount: ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut input).unwrap();
    // let value: f64 = input.trim().parse().unwrap();

    // let option = print_options(value);

    // match option {
    //     1 => println!("You've choose option 1"),
    //     2 => println!("You've choose option 2"),
    //     3 => println!("You've choose option 3"),
    //     _ => println!("Invalid option"),
    // }

    // fn print_options(value: f64) -> u32 {
    //     let full = infull(value);
    //     let half = inhalf(value);
    //     let interest = 0.03;

    //     print!(
    //         "Available payment options:\n\
    //         Option 1: full payment with discount of 10% (US$ {full:.2})\n\
    //         Option 2: half-payment in two installments of US$ {half:.2}\n"
    //     );

    //     if value > 100_f64 {
    //         println!("Option 3: from 3 until 10 installments of:");
    //         for i in 3..=10 {
    //             let installments = in3to10(value, interest, i);
    //             println!("\t{i}x of US$ {:.2}", installments);
    //         }
    //     }

    //     let mut input = String::new();
    //     input.clear();
    //     print!("Type an option number: ");
    //     io::stdout().flush().unwrap();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let option: u32 = input.trim().parse().unwrap();

    //     option
    // }

    // fn inhalf(value: f64) -> f64 {
    //     value / 2.0
    // }

    // fn infull(value: f64) -> f64 {
    //     value * (1.0 - 0.1)
    // }

    // fn in3to10(pv: f64, i: f64, time: i32) -> f64 {
    //     let x = (1.0 + i).powi(time);

    //     pv * i * x / (x - 1_f64)
    // }
    // group vi
    // 1
    // struct Number {
    //     minor: f64,
    //     major: f64,
    // }

    // let times = 4;
    // let mut values: Vec<f64> = Vec::new();
    // let mut input = String::new();
    // for i in 0..times {
    //     print!("Type the value {i}: ");
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let value: f64 = input.trim().parse().unwrap();
    //     values.push(value);
    // }

    // let nums: Number = major(values);

    // println!("minor: {}, major: {}", nums.minor, nums.major);

    // fn major(values: Vec<f64>) -> Number {
    //     let mut min: f64 = values[0];
    //     let mut max: f64 = values[0];
    //     for value in values {
    //         min = if min < value { min } else { value };
    //         max = if max > value { max } else { value };
    //     }

    //     Number {
    //         minor: min,
    //         major: max,
    //     }
    // }

    // 2
    // let times = 4;
    // let mut values: Vec<f64> = Vec::new();
    // let mut input = String::new();
    // for i in 0..times {
    //     print!("Type the value {}: ", i + 1);
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let value: f64 = input.trim().parse().unwrap();
    //     values.push(value);
    // }

    // let values = bubblesort(&mut values);

    // println!("{:?}", values);

    // fn bubblesort(values: &mut Vec<f64>) -> &mut Vec<f64> {
    //     let n = values.len();
    //     for i in 0..n {
    //         for j in 0..n - 1 - i {
    //             if values[j] > values[j + 1] {
    //                 values.swap(j, j + 1);
    //             }
    //         }
    //     }

    //     values
    // }

    // 3
    // let times = 4;
    // let mut values: Vec<f64> = Vec::new();
    // let mut input = String::new();
    // for i in 0..times {
    //     print!("Type the value {}: ", i + 1);
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let value: f64 = input.trim().parse().unwrap();
    //     values.push(value);
    // }

    // println!("Medium: {}", medium(values));

    // fn medium(values: Vec<f64>) -> f64 {
    //     let len = values.len();
    //     let mut sum: f64 = 0.0;
    //     for value in values {
    //         sum += value;
    //     }

    //     if len != 0 { sum / len as f64 } else { sum }
    // }

    // 4
    // let mut input: String = String::new();
    // let mut values: Vec<f64> = Vec::new();

    // print!("Type how many values do you want to place in the array: ");
    // io::stdout().flush().unwrap();
    // input.clear();
    // io::stdin().read_line(&mut input).unwrap();
    // let options: i32 = input.trim().parse().unwrap();

    // for i in 1..=options {
    //     print!("Type the option {i}: ");
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let value: f64 = input.trim().parse().unwrap();
    //     values.push(value);
    // }

    // println!("{:?}", values);

    // 5
    // let mut input = String::new();
    // let mut values: Vec<i32> = Vec::new();
    // let mut sum = 0;
    // for i in 0..10 {
    //     print!("Type the value {}: ", i + 1);
    //     io::stdout().flush().unwrap();
    //     input.clear();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let value: i32 = input.trim().parse().unwrap();
    //     values.push(value);
    //     sum += value;
    // }

    // let medium: f64 = sum as f64 / 10_f64;

    // println!("Medium: {medium}\nSum: {sum}");
    // let mut member = 0;
    // for value in values {
    //     if medium == value as f64 {
    //         member = value;
    //         break;
    //     }
    // }

    // println!("Yep, {member} is equals to the medium of all members");

    // 6
    // let rows = 5;
    // let cols = 3;
    // let mut even_medium = 0_f64;
    // let mut odds_medium = 0_f64;
    // let mut input = String::new();
    // let mut matrix: Vec<Vec<f64>> = vec![vec![0_f64; cols]; rows];
    // for (i, _) in (0..rows).enumerate() {
    //     for (j, _) in (0..cols).enumerate() {
    //         print!("Type the value [{i}][{j}]: ");
    //         io::stdout().flush().unwrap();
    //         input.clear();
    //         io::stdin().read_line(&mut input).unwrap();
    //         let value: f64 = input.trim().parse().unwrap();
    //         matrix[i][j] = value;
    //         if j % 2 == 0 {
    //             odds_medium += matrix[i][j];
    //         } else {
    //             even_medium += matrix[i][j];
    //         }
    //     }
    // }

    // even_medium /= cols as f64;
    // odds_medium /= cols as f64;

    // // println!("{:?}", matrix);
    // println!(
    //     "Odd columns medium: {odds_medium}\n\
    //     Even columns medium: {even_medium}"
    // );
}
