use impls::*;

fn main() {
    let triangle = Triangle {
        base: 10_f64,
        height: 20_f64,
    };

    let square = Square { side: 2.0 };
    let rectangle = Rectangle {
        base: 4.0,
        height: 5.4,
    };
    let circle = Circle { radius: 5.46 };

    print_area(&triangle);
    print_area(&rectangle);
    print_area(&square);
    print_area(&circle);
}

fn print_area<T: Area>(shape: &T) {
    println!("{}", shape.area());
}
