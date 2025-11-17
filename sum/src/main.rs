fn main() {
    let numbers = "10, 20, 30, 40, 50, 60, 70, 80, 90, 100";

    let obj_numbers: Vec<i32> = numbers
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // println!("{:?}", obj_numbers);
    for i in obj_numbers {
        println!("{i}")
    }
}
