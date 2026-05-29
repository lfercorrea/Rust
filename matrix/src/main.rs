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

        for x in 0..rows {
            for y in 0..cols {
                let n = get_f64("Type next matrix element: ");
                matrix[x * cols + y] = n;
            }
        }

        Self {
            data: matrix,
            height: rows,
            width: cols,
        }
    }

    fn det(&self) {
        let mut m = self.clone();

        for pivot in 0..self.height {
            for row in (pivot + 1)..self.height {
                //TODO: lookup for greater pivot in the triangular matrix
                let factor = m.data[row * m.width + pivot] / m.data[pivot * m.width + pivot];
                for col in pivot..self.width {
                    m.data[row * self.width + col] -= factor * m.data[pivot * m.width + col];
                }
            }
        }
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
    // let rows = get_i32("Tipe matrix rows: ");
    // let rows = rows as usize;
    // let cols = get_i32("Tipe matrix cols: ");
    // let cols = cols as usize;

    // let matrix = Matrix::new(rows, cols);

    let mtx_a = Matrix {
        data: vec![2.0, 3.0, 4.0, 1.0, 0.0, 0.0],
        height: 2,
        width: 3,
    };
    let mtx_b = Matrix {
        data: vec![0.0, 1000.0, 1.0, 100.0, 0.0, 10.0],
        height: 3,
        width: 2,
    };

    let mtx_c = mtx_a.mul(mtx_b);

    mtx_c.print();
}
