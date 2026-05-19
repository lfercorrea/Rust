use std::{fmt::Display, vec};

pub struct Primes {
    primes: Vec<u64>,
}

impl Display for Primes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let primes = self
            .primes
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{primes}")
    }
}

impl Primes {
    pub fn erathostenes(n: usize) -> Primes {
        let mut _bools = vec![true; n];
        _bools[0] = false;
        _bools[1] = false;

        for i in 2..=((n as f64).sqrt() as usize) {
            if _bools[i] {
                for j in ((i * i)..n).step_by(i) {
                    _bools[j] = false;
                }
            }
        }

        let mut primes: Vec<u64> = Vec::new();

        for (key, val) in _bools.iter().enumerate() {
            if *val {
                primes.push(key as u64);
            }
        }

        Primes { primes }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
