pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub base: f64,
    pub height: f64,
}

pub struct Square {
    pub side: f64,
}

pub struct Triangle {
    pub height: f64,
    pub base: f64,
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.base * self.height
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2_f64
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
