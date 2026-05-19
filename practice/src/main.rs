use myrustlib::{self, get_string};

fn main() {
    let input = get_string("Type something...: ");

    let positions: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();

    for string in positions {
        println!("{string}");
    }
}
