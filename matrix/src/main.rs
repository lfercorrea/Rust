use std::hint::select_unpredictable;

use myrustlib::get_f64;

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<f64>,
    height: usize,
    width: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
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

    fn sum(&self, rhs_mtx: Matrix) -> Self {
        assert!(
            self.height == rhs_mtx.height && self.width == rhs_mtx.width,
            "The matrixes has incompatible dimensions to sum together."
        );
        let mut new_mtx = vec![0.0; rhs_mtx.height * rhs_mtx.width];

        for i in 0..self.height {
            for j in 0..self.width {
                new_mtx[i * self.width + j] =
                    self.data[i * self.width + j] + rhs_mtx.data[i * self.width + j];
            }
        }

        Self {
            data: new_mtx,
            height: self.height,
            width: self.width,
        }
    }

    fn mul(&self, rhs_mtx: Matrix) -> Self {
        assert!(
            self.width == rhs_mtx.height,
            "The matrixes haven't compatible dimensions to perform a multiplication."
        );

        let mut new_mtx = vec![0.0; self.height * rhs_mtx.width];

        for i in 0..self.height {
            for j in 0..rhs_mtx.width {
                for k in 0..self.width {
                    new_mtx[i * rhs_mtx.width + j] +=
                        self.data[i * self.width + k] * rhs_mtx.data[k * rhs_mtx.width + j];
                }
            }
        }

        Self {
            data: new_mtx,
            height: self.height,
            width: rhs_mtx.width,
        }
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

    fn print(&self) {
        let mut l_edge = true;
        for i in 0..self.height {
            for j in 0..self.width {
                if l_edge {
                    print!("|\t");
                    l_edge = false;
                }

                print!("{:.2}", self.data[i * self.width + j]);

                if j > 0 || j < self.width {
                    print!("\t")
                }

                if j == self.width - 1 {
                    println!("|");
                    l_edge = true;
                }
            }
        }
    }
}

fn main() {
    let mtx_a = Matrix::new(3, 3);
    println!("Matriz original: ");
    mtx_a.print();

    let scalar = 3.0;
    let scaled = mtx_a.scale(scalar);
    println!("Scaled matrix * {:.2} =", scalar);
    scaled.print();
}
