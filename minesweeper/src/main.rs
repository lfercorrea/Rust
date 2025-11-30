use myrustlib::{self, get_i32};
use rand::{self, Rng};

#[derive(Clone)]
struct Cell {
    open: bool,
    contains_bomb: bool,
    interrogation: bool,
    neighbor: u32,
}

struct State {
    remaining_cells: u32,
    remaining_bombs: u32,
    discovered_cells: u32,
}

fn main() {
    let board_size = get_i32("Type the board size (e.g. 8): ") as usize;
    let bombs = get_i32("Type thow many bombs would you like to put in the board: ") as u32;
    if bombs as usize >= (board_size * board_size) || bombs <= 0 {
        panic!("Doesn't make any sense put amount of bombs in a minesweeper game. Exiting...")
    }

    let cell = Cell {
        open: false,
        contains_bomb: false,
        interrogation: false,
        neighbor: 0,
    };
    let mut state = State {
        remaining_cells: (board_size * board_size) as u32,
        remaining_bombs: bombs,
        discovered_cells: 0,
    };
    let mut board: Vec<Vec<Cell>> = vec![vec![cell; board_size]; board_size];

    set_bombs(bombs, board_size, &mut board);
    print_board(&board, board_size, state);
}

fn set_bombs(bombs: u32, board_size: usize, board: &mut Vec<Vec<Cell>>) {
    let mut rng = rand::rng();
    let mut planted = 0;
    while planted < bombs {
        let r: usize = rng.random_range(0..board_size);
        let c: usize = rng.random_range(0..board_size);

        if !board[r][c].contains_bomb {
            board[r][c].contains_bomb = true;
            planted += 1;
        }
    }

    for r in 0..board_size {
        for c in 0..board_size {
            board[r][c].neighbor = count_neighbor(r, c, board, board_size);
        }
    }
}

fn print_board(board: &[Vec<Cell>], board_size: usize, state: State) {
    println!("\x1b[2J\x1b[H\x1b[1;37mCurrent Board State:\x1b[0m ");
    for i in 0..board_size {
        for j in 0..board_size {
            if j % board_size == 0 {
                println!()
            }

            print!(" ");
            if board[i][j].contains_bomb {
                print!("\x1b[1;31mB\x1b[0m")
            } else if board[i][j].interrogation {
                print!("\x1b[1;35m?\x1b[0m")
            } else {
                let cell = &board[i][j];
                if cell.neighbor == 0 {
                    print!(".")
                } else {
                    print!("{}", cell.neighbor);
                }
            }
        }
    }

    println!("\n");
    println!(
        "\x1b[1;31mRemainning bombs: \x1b[1;37m{}\x1b[0m",
        state.remaining_bombs
    );
    println!(
        "\x1b[1;32mDiscovered cells: \x1b[1;37m{}\x1b[0m",
        state.discovered_cells
    );
    println!(
        "\x1b[1;36mRemainning cells: \x1b[1;37m{}\x1b[0m",
        state.remaining_cells
    );
}

fn count_neighbor(row: usize, col: usize, board: &[Vec<Cell>], board_size: usize) -> u32 {
    let mut count = 0;
    for r in -1isize..=1 {
        for c in -1isize..=1 {
            if r == 0 && c == 0 {
                continue;
            }

            let row_idx = row as isize + r;
            let col_idx = col as isize + c;

            if row_idx >= 0
                && col_idx >= 0
                && (row_idx as usize) < board_size
                && (col_idx as usize) < board_size
            {
                let cell = &board[row_idx as usize][col_idx as usize];
                if cell.contains_bomb {
                    count += 1;
                }
            }
        }
    }

    count
}
