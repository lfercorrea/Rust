fn main() {
    let numbers = "10, 20, 30, 40, 50, 60, 70, 80, 90, 100";
    let obj_numbers = split_numbers(numbers);
    let mut sum: i32 = 0;
    println!("Targets: \x1b[1;32m{:?}\x1b[0m", obj_numbers);
    for i in obj_numbers {
        sum += i;
    }
    println!("The sum is: \x1b[1;32m{sum}\x1b[0m")
}

fn split_numbers(s: &str) -> Vec<i32> {
    let obj_numbers: Vec<i32> = s
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    obj_numbers
}
