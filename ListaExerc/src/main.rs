use core::num;
use std::{
    arch::x86_64::_MM_FLUSH_ZERO_MASK,
    ffi::c_double,
    fs::read_link,
    io::{self, Read, Write, empty, stdin, stdout},
    result, string,
};

enum Value {
    IntegerT(i32),
    DoubleT(f64),
    StringT(String),
}

fn get_string(input_msg: &str) -> String {
    let mut input = String::new();
    print!("{input_msg}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim_end().to_string()
}

fn get_int(input_msg: &str) -> i32 {
    let mut input = String::new();
    print!("{input_msg}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap()
}

fn get_double(input_msg: &str) -> f64 {
    let mut input = String::new();
    print!("{input_msg}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap()
}

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

    // group vii - strings
    // 1
    // let mut input = String::new();
    // print!("Type a string: ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut input).unwrap();
    // let text: String = input.trim_end().chars().take(80).collect();
    // let len = text.chars().count();
    // let mut punctuation = 0;
    // let mut lower = 0;
    // let mut numeric = 0;

    // for c in text.chars() {
    //     if c.is_ascii_punctuation() {
    //         punctuation += 1;
    //     }
    //     if c.is_ascii_digit() {
    //         numeric += 1;
    //     }
    //     if c.is_ascii_lowercase() {
    //         lower += 1;
    //     }
    // }

    // println!(
    //     "Str len: {len}\n\
    //     Punctuation chars: {punctuation}\n\
    //     Numeric chars: {numeric}\n\
    //     Lower chars: {lower}\n"
    // );

    // 2
    // let text = get_string("Escolha uma string: ");

    // for word in text.split(' ') {
    //     println!("{word}");
    // }

    // 3
    // let input = get_string("Type a string: ");
    // let input = input.trim_end();
    // let chars: Vec<char> = input.chars().collect();

    // print_normal(&chars, 0);
    // println!();
    // print_reverse(&chars, chars.len());
    // println!();
    // fn print_normal(chars: &[char], idx: usize) {
    //     if idx >= chars.len() {
    //         return;
    //     }
    //     print!("{}", chars[idx]);

    //     print_normal(chars, idx + 1)
    // }

    // fn print_reverse(chars: &[char], idx: usize) {
    //     if idx == 0 {
    //         return;
    //     }
    //     print!("{}", chars[idx - 1]);

    //     print_reverse(chars, idx - 1);
    // }

    // 4
    // let name = get_string("Type your name: ");
    // let address = get_string("Type your address: ");
    // let phone = get_string("Type your phone number: ");
    // let age = get_int("Type your age (years): ");

    // println!(
    //     "Your name is {name}.\n\
    //     Your address is {address}.\n\
    //     Your phone number is {phone} and you're {age} years old."
    // );

    // 5
    // match strbool() {
    //     Some(value) => println!("Value: {value}"),
    //     None => println!("No return"),
    // }

    // fn strbool() -> Option<i32> {
    //     let input = get_string("Type a string: ");
    //     match input.as_str() {
    //         "SIM" => Some(1),
    //         "NÃO" => Some(0),
    //         _ => None,
    //     }
    // }

    // 6
    // let input = get_string("Type a string: ");
    // print_words(input);

    // fn print_words(input: String) {
    //     println!(
    //         "'{}' has {} words.",
    //         input,
    //         input.split_whitespace().count()
    //     );
    // }

    // 7
    // let mut emptystr = String::new();
    // let text = get_string("Type a text: ");
    // strcpy(&mut emptystr, text);

    // println!("Copied text: '{emptystr}'");

    // fn strcpy(dest: &mut String, src: String) {
    //     let chars: Vec<char> = src.chars().collect();

    //     for c in chars {
    //         dest.push(c);
    //     }
    // }

    // 8
    // let input = get_string("Type any number: ");
    // let numbers_ext: [&str; 10] = [
    //     "zero", "um", "dois", "três", "quatro", "cinco", "seis", "sete", "oito", "nove",
    // ];

    // for ch in input.chars() {
    //     let c: u8 = ch as u8 - b'0';
    //     print!("{}, ", numbers_ext[c as usize]);
    // }

    // 9
    // let mut names: Vec<String> = Vec::new();
    // let mut i = 0;
    // while i < 20 {
    //     let name = get_string("Type a name and his lastname: ");
    //     if name == "FIM" {
    //         break;
    //     }
    //     names.push(name);
    //     i += 1;
    // }

    // let mut lastnames = names.clone();
    // lastnames.sort_by(|a, b| {
    //     let last_a = a.split_whitespace().last().unwrap_or("");
    //     let last_b = b.split_whitespace().last().unwrap_or("");

    //     // Se os sobrenomes forem iguais, ordena pelo nome completo
    //     last_a.cmp(last_b).then(a.cmp(b))
    // });
    // names.sort();

    // println!("Sorted by name: {:?}", names);
    // println!("Sorted by lastname: {:?}", lastnames);

    // 10
    // let mut name = get_string("Type yout name, we'll remove all 'a' occurences from it: ");
    // removechar(&mut name, 'a');
    // println!("{name}");
    // fn removechar(input: &mut String, c: char) {
    //     input.retain(|ch| ch != c);
    // }

    // 11
    // let mut input = get_string("Type anything: ");
    // addchar(&mut input, 'A', 7);
    // println!("{input}");
    // fn addchar(input: &mut String, c: char, pos: usize) {
    //     input.insert(pos, c);
    // }

    //12 skipped
    // let result = conv2dec("345".to_string(), 8);
    // println!("{result}");
    // fn conv2dec(number_str: String, base_int: i32) -> i32 {
    //     let numbers: Vec<char> = number_str.chars().collect();
    //     let mut total = 0;
    //     for (i, c) in numbers.iter().rev().enumerate() {
    //         let number = char_to_value(*c);
    //         total += number * base_int.pow(i as u32);
    //     }

    //     total
    // }

    // fn char_to_value(c: char) -> i32 {
    //     match c {
    //         '0'..='9' => (c as u8 - b'0') as i32,
    //         'A'..='Z' => (c as u8 - b'A' + 10) as i32,
    //         'a'..='z' => (c as u8 - b'a' + 36) as i32,
    //         _ => panic!("INVALID CHAR"),
    //     }
    // }

    /*
         Grupo VIII - Passagem de parâmetros e estruturas
    1. Fazer uma função que retorna a soma, a diferença e o produto entre dois
    números.
    */
    // struct Numbers {
    //     sum: i32,
    //     diff: i32,
    //     prod: i32,
    // }

    // let result = mymath(3, 4);

    // println!(
    //     "Sum: {}\n\
    //     Diff: {}\n\
    //     Prod: {}",
    //     result.sum, result.diff, result.prod
    // );

    // fn mymath(n1: i32, n2: i32) -> Numbers {
    //     Numbers {
    //         sum: n1 + n2,
    //         diff: n1 - n2,
    //         prod: n1 * n2,
    //     }
    // }

    /*
        2. Fazer uma função em "rust" que retorna a razão entre dois números. A função
    deve retornar pelo comando return o valor 1 se a operação foi possível e o
    valor 0 se a operação não foi possível (divisão por zero, por exemplo). O
    resultado da divisão deve retonar por um parâmetro por referência.
         */

    // let mut div = 0_f64;
    // let dest = ratio(&mut div, 8.0, 2.0);

    // println!(
    //     "Div = {}\n\
    //     Return = {}",
    //     div, dest
    // );

    // fn ratio(dest: &mut f64, n1: f64, n2: f64) -> f64 {
    //     if n2 == 0.0 {
    //         return 0.0;
    //     }
    //     *dest = n1 / n2;

    //     1.0
    // }

    /*
        3. Fazer uma rotina em "rust" que recebe um vetor de números inteiros como
    parâmetro onde todos os valores exceto o último são positivos e devolve:
        - a média dos valores do vetor;
        - o menor valor do vetor (sem considerar o último)
        - o maior valor do vetor
     */
    // let numbers: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, -9];

    // struct Numbers {
    //     sum: i32,
    //     len: usize,
    //     med: f64,
    //     min: i32,
    //     max: i32,
    // }

    // let n = mathoper(numbers);

    // println!(
    //     "Sum: {}\n\
    //     Len: {}\n\
    //     Med: {:.2}\n\
    //     Min: {}\n\
    //     Max: {}",
    //     n.sum, n.len, n.med, n.min, n.max
    // );

    // fn mathoper(numbers: [i32; 10]) -> Numbers {
    //     let len = numbers.len();
    //     let mut min = numbers[0];
    //     let mut max = numbers[0];
    //     let mut sum = 0;

    //     for n in numbers {
    //         if n > 0 {
    //             if min > n {
    //                 min = n;
    //             }

    //             if max < n {
    //                 max = n;
    //             }

    //             sum += n;
    //         }
    //     }

    //     let med = sum as f64 / len as f64;

    //     Numbers {
    //         sum,
    //         len,
    //         med,
    //         min,
    //         max,
    //     }
    // }

    // /*
    //    4. Fazer uma função para ler e retornar o valor das 3 notas de um aluno.
    // */
    // let mut grades: Vec<f64> = Vec::new();

    // for i in 0..3 {
    //     let grade = get_double("Type an grade: ");
    //     grades.push(grade);
    // }

    // println!("Grades: {:?}", grades);

    /*
        5. Construir um programa em "C" que implementa uma agenda eletrônica. O
    programa deve ter um menu com as seguintes opções:
        Entrar um novo nome na agenda.
        Imprimir na tela os dados de uma das pessoas cadastradas (conforme
        solicitação).
        Imprimir a lista de nomes cadastrados que comecem pela letra indicada.
        Fim
    Cada entrada da agenda deve ter os seguintes campos:
        char nome[30];
        char endereco[100];
        char fone[10];
        long int CEP;
        Obs: a agenda deve ter capacidade para 100 entradas.
         */
    struct Contato {
        nome: String,
        endereco: String,
        fone: String,
        cep: i32,
    }

    let mut agenda: Vec<Contato> = Vec::new();

    loop {
        println!(
            "Menu principal do programa:\n\n\
            1. Entrar um novo nome na agenda\n\
            2. Imprimir na tela os dados de uma das pessoas cadastradas\n\
            3. Imprimir a lista de nomes cadastrados que comecem pela letra indicada\n\
            4. Fim"
        );

        let option: i32 = get_int("Escolha uma opção: ");
        match option {
            1 => {
                let contato: Contato = get_person();
                agenda.push(contato);
            }
            2 => {
                for contato in &agenda {
                    println!("nome: {}", contato.nome);
                }
            }
            3 => {
                let letra = get_string("Digite uma letra para listar: ");
                for contato in &agenda {
                    if contato.nome.starts_with(&letra) {
                        println!("nome: {}", contato.nome);
                    }
                }
            }
            4 => {
                println!("Saindo...");
                return;
            }
            _ => panic!("Opção inválida"),
        }
    }

    fn get_person() -> Contato {
        let nome = get_string("Insira o nome do contato: ");
        let endereco = get_string("Insira o endereço do contato: ");
        let cep = get_int("Insira o cep do contato: ");
        let fone = get_string("Insira o telefone do contato: ");

        Contato {
            nome,
            endereco,
            fone,
            cep,
        }
    }
}
