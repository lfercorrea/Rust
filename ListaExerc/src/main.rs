use std::{ffi::c_double, io, result};

fn main() {
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
    let hex = "0123456789ABCDEF".as_bytes();
    let mut buffer = String::new();
    let mut result = String::new();
    let mut idx: usize;
    let mut n: u32 = 2983;

    while n > 0 {
        idx = (n % 16) as usize;
        buffer.push(hex[idx] as char);
        n /= 16;
    }

    for ch in buffer.chars().rev() {
        result.push(ch);
    }

    println!("{buffer}");
    println!("{result}");
}
