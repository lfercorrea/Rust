use std::ops::{Add, Mul, Sub};

use myrustlib::get_f64;

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<f64>,
    height: usize,
    width: usize,
}

const EPS: f64 = 1e-12;

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for i in 0..self.height {
            for j in 0..self.width {
                if j == 0 {
                    write!(f, "|\t")?;
                }

                write!(f, "{:.2}", self.data[i * self.width + j])?;

                if j > 0 || j < self.width {
                    write!(f, "\t")?;
                }

                if j == self.width - 1 {
                    writeln!(f, "|")?;
                }
            }
        }

        writeln!(f)
    }
}

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(
            self.height == rhs.height && self.width == rhs.width,
            "Incompatible dimensions to perform a sum"
        );

        let mut sum = Matrix {
            data: vec![0_f64; self.height * self.width],
            height: self.height,
            width: self.width,
        };

        for i in 0..sum.height {
            for j in 0..sum.width {
                sum.data[i * sum.width + j] =
                    self.data[i * self.width + j] + rhs.data[i * rhs.width + j];
            }
        }

        sum
    }
}

impl Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            self.height == rhs.height && self.width == rhs.width,
            "Incompatible dimensions to perform a subtraction"
        );

        let mut sub = Matrix {
            data: vec![0_f64; self.height * self.width],
            height: self.height,
            width: self.width,
        };

        for i in 0..sub.height {
            for j in 0..sub.width {
                sub.data[i * sub.width + j] =
                    self.data[i * self.width + j] - rhs.data[i * rhs.width + j];
            }
        }

        sub
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(
            self.width == rhs.height,
            "The matrixes have incompatible dimenions to multiply"
        );

        let mut mul = Matrix {
            data: vec![0_f64; self.height * rhs.width],
            height: self.height,
            width: rhs.width,
        };

        for i in 0..self.height {
            for j in 0..rhs.width {
                for k in 0..self.width {
                    mul.data[i * mul.width + j] +=
                        self.data[i * self.width + k] * rhs.data[k * rhs.width + j];
                }
            }
        }

        mul
    }
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        assert!(
            rows > 0 && cols > 0,
            "Impossible matrix dimensions, row and col must be greater than zero"
        );
        let mut matrix = vec![0.0; rows * cols];
        // let matrix: Vec<Vec<i64>> = (0..rows).map(|_| (0..cols).map(|_| 0).collect()).collect();

        for i in 0..rows {
            for j in 0..cols {
                let msg = format!(
                    "Type next matrix element [linha: {}][coluna: {}]: ",
                    i + 1,
                    j + 1
                );
                let n = get_f64(msg.as_str());
                matrix[i * cols + j] = n;
            }
        }

        Self {
            data: matrix,
            height: rows,
            width: cols,
        }
    }

    fn det(&self) -> f64 {
        assert!(self.height == self.width);

        let mut m = self.clone();
        let mut det = 1.0;

        for pivot in 0..self.height {
            let mut best_abs = m.data[pivot * m.width + pivot].abs();
            let mut best_row = pivot;

            for row in (pivot + 1)..self.height {
                let value = m.data[row * m.width + pivot].abs();

                if value > best_abs {
                    best_abs = value;
                    best_row = row;
                }
            }

            if best_abs == 0.0 {
                return 0.0;
            }

            if best_row != pivot {
                for col in 0..self.width {
                    m.data.swap(pivot * m.width + col, best_row * m.width + col);
                }

                det = -det;
            }

            for row in (pivot + 1)..self.height {
                let factor = m.data[row * m.width + pivot] / m.data[pivot * m.width + pivot];

                for col in pivot..self.width {
                    m.data[row * m.width + col] -= factor * m.data[pivot * m.width + col];
                }
            }
        }

        for i in 0..self.height {
            det *= m.data[i * m.width + i];
        }

        det
    }

    fn div(&self, rhs: &Matrix) -> Option<Matrix> {
        if let Some(matrix) = rhs.inverse() {
            return Some(self + &matrix);
        }

        None
    }

    fn identity(&self) -> Option<Matrix> {
        if self.height != self.width {
            return None;
        }

        let data: Vec<f64> = (0..self.height * self.width)
            .map(|i| {
                let row = i / self.width;
                let col = i % self.width;

                if row == col { 1.0 } else { 0.0 }
            })
            .collect();

        Some(Matrix {
            data,
            height: self.height,
            width: self.width,
        })
    }

    fn inverse(&self) -> Option<Matrix> {
        let mut idt = self.identity().unwrap();
        let mut m = self.clone();

        for pivot in 0..m.height {
            let mut best_abs = m.data[pivot * m.width + pivot].abs();
            let mut best_row = pivot;

            for row in (pivot + 1)..m.height {
                let value = m.data[row * m.width + pivot].abs();

                if value > best_abs {
                    best_abs = value;
                    best_row = row;
                }
            }

            if best_row != pivot {
                for col in 0..m.width {
                    m.data.swap(pivot * m.width + col, best_row * m.width + col);
                    idt.data
                        .swap(pivot * idt.width + col, best_row * idt.width + col);
                }
            }

            if best_abs < EPS {
                return None;
            }

            let pivot_value = m.data[pivot * m.width + pivot];

            for col in 0..m.width {
                m.data[pivot * m.width + col] /= pivot_value;
                idt.data[pivot * idt.width + col] /= pivot_value;
            }

            for row in (pivot + 1)..m.height {
                let factor = m.data[row * m.width + pivot] / m.data[pivot * m.width + pivot];

                for col in pivot..m.width {
                    m.data[row * m.width + col] -= factor * m.data[pivot * m.width + col];
                }

                for col in 0..m.width {
                    idt.data[row * idt.width + col] -= factor * idt.data[pivot * idt.width + col];
                }
            }
        }

        for pivot in (0..m.height).rev() {
            for row in 0..pivot {
                let factor = m.data[row * m.width + pivot];

                for col in 0..m.width {
                    m.data[row * m.width + col] -= factor * m.data[pivot * m.width + col];
                    idt.data[row * idt.width + col] -= factor * idt.data[pivot * idt.width + col];
                }
            }
        }

        Some(idt)
    }

    fn is_simetric(&self) -> bool {
        if self.height != self.width {
            return false;
        }

        let tranposed = self.transpose();

        for i in 0..self.height {
            for j in 0..self.width {
                if tranposed.data[i * tranposed.width + j] != self.data[i * self.width + j] {
                    return false;
                }
            }
        }

        true
    }

    fn scale(&self, scalar: f64) -> Matrix {
        let mut new_mtx = self.clone();

        for i in 0..new_mtx.height {
            for j in 0..new_mtx.width {
                new_mtx.data[i * new_mtx.width + j] *= scalar;
            }
        }

        new_mtx
    }

    fn transpose(&self) -> Matrix {
        let mut transposed = Matrix {
            data: vec![0.0; self.height * self.width],
            height: self.width,
            width: self.height,
        };

        for i in 0..self.height {
            for j in 0..self.width {
                transposed.data[j * transposed.width + i] = self.data[i * self.width + j];
            }
        }

        transposed
    }
}

fn main() {
    let a = Matrix::new(2, 2);
    println!("Matrix A: {}", a);
    let b = Matrix::new(2, 2);
    println!("Matrix B: {}", b);
    let c = &a * &b;
    println!("(A*B) = Matrix C: {}", c);
}
