use myrustlib::{self, get_string};

fn main() {
    let input = get_string("Type something...: ");

    let positions: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();

    for string in positions {
        println!("{string}");
    }

    let n = input.parse();

    match n {
        Ok(n) => {
            fac(n);
        }
        Err(e) => panic!("Invalid input for convert into number: {e}"),
    }
}

fn fac(n: u64) {
    let mut prod = 1;
    for i in 1..=n {
        prod *= i;
    }

    println!("{prod}")
}
