use myrustlib::get_i32;
use std::io;

fn main() {
    println!("Type matrix rows: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let lines: usize = input.trim().parse().unwrap();

    println!("Type matrix cols: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cols: usize = input.trim().parse().unwrap();

    let mut matrix: Vec<Vec<i64>> = Vec::new();
    // let matrix: Vec<Vec<i64>> = (0..lines).map(|_| (0..cols).map(|_| 0).collect()).collect();

    for _ in 0..lines {
        let mut line: Vec<i64> = Vec::new();
        for _ in 0..cols {
            let n = get_i32("Type the matrix item:");
            line.push(n as i64);
        }
        matrix.push(line);
    }

    println!("matrix: {:?}", matrix)
}
