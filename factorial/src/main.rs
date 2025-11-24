use myrustlib::{self, get_i32};

fn main() {
    let n = get_i32("Type a number: ");
    let fac = facrec(n);
    println!("{n}! = {fac}");
}

fn facite(n: i32) -> i64 {
    let mut fac: i64 = 1;
    let mut a;

    for i in 2..=n {
        a = i;
        fac *= a as i64;
    }

    fac
}

fn facrec(n: i32) -> i64 {
    if n <= 2 {
        return n as i64;
    }

    n as i64 * facrec(n - 1)
}
