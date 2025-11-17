fn main() {
    for i in 0..20 {
        let fib = recfib(i);
        println!("{fib}")
    }
}

fn fibonacci(n: i64) -> i64 {
    if n < 2 {
        return n;
    }

    let mut a: i64 = 0;
    let mut b: i64 = 1;
    let mut i: i64 = 2;
    let mut fib: i64 = 1;

    while i <= n {
        fib = a + b;
        a = b;
        b = fib;
        i += 1;
    }

    fib
}

fn recfib(n: i64) -> i64 {
    if n < 2 {
        return n;
    }

    recfib(n - 1) + recfib(n - 2)
}
