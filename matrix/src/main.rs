use myrustlib::get_i32;
use std::io;

struct Matrix(Vec<i64>);

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        let mut matrix = vec![0; rows * cols];
        // let matrix: Vec<Vec<i64>> = (0..rows).map(|_| (0..cols).map(|_| 0).collect()).collect();

        for x in 0..rows {
            for y in 0..cols {
                let n = get_i32("Type next matrix element: ");
                matrix[x * cols + y] = n as i64;
            }
        }

        Self(matrix)
    }

    fn print_mtx(&self, height: usize, width: usize) {
        let mut l_edge = true;
        for i in 0..height {
            for j in 0..width {
                if l_edge {
                    print!("|\t");
                    l_edge = false;
                }

                print!("{}", self.0[i * width + j]);

                if j > 0 || j < width {
                    print!("\t")
                }

                if j == width - 1 {
                    println!("|");
                    l_edge = true;
                }
            }
        }
    }
}

fn main() {
    let rows = get_i32("Tipe matrix rows: ");
    let rows = rows as usize;
    let cols = get_i32("Tipe matrix cols: ");
    let cols = cols as usize;

    let matrix = Matrix::new(rows, cols);

    // println!("matrix: {:?}", matrix.0)
    matrix.print_mtx(rows, cols);
}
