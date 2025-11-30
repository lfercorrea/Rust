use std::os::linux::raw::stat;

use myrustlib::{self, get_char, get_i32};
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
    game_over: bool,
}

fn main() {
    loop {
        let board_size = get_i32("Type the board size (e.g. 8): ") as usize;
        let bombs = get_i32("Type thow many bombs would you like to put in the board: ") as u32;
        if bombs as usize >= (board_size * board_size) {
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
            game_over: false,
        };
        let mut board: Vec<Vec<Cell>> = vec![vec![cell; board_size]; board_size];

        set_bombs(bombs, board_size, &mut board);
        print_board(&board, board_size, &state);

        loop {
            let row = get_i32("\x1b[1;35mType the row number:\x1b[0m ");
            let col = get_i32("\x1b[1;34mNow, type the col number:\x1b[0m ");
            open_cell(
                &mut board,
                board_size,
                row as usize - 1,
                col as usize - 1,
                &mut state,
            );
            print_board(&board, board_size, &state);
            if state.game_over {
                let play = get_char("Would you like to play again? (y/n): ");
                if play == 'n' {
                    return;
                } else {
                    break;
                }
            }
        }
    }
}

fn set_bombs(bombs: u32, board_size: usize, board: &mut [Vec<Cell>]) {
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

fn print_board(board: &[Vec<Cell>], board_size: usize, state: &State) {
    println!("\x1b[2J\x1b[H\x1b[1;37mCurrent Board State:\x1b[0m ");

    let index_width = board_size.to_string().len();
    print!("{:index_width$}", "");

    for j in 1..=board_size {
        print!(" \x1b[1;34m{:>index_width$}\x1b[0m", j);
    }
    println!();

    for (i, row) in board.iter().enumerate() {
        print!("\x1b[1;35m{:>index_width$}\x1b[0m", i + 1);
        for cell in row {
            let symbol = if cell.interrogation {
                "?"
            } else if cell.open || state.game_over {
                if cell.contains_bomb {
                    "B"
                } else {
                    &cell.neighbor.to_string()
                }
            } else {
                "."
            };
            print!(" {:>index_width$}", symbol);
        }
        println!();
    }

    println!("\n");
    println!(
        "\x1b[1;31mPlanted bombs: \x1b[1;37m{}\x1b[0m",
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
    println!();
}

fn count_neighbor(row: usize, col: usize, board: &mut [Vec<Cell>], board_size: usize) -> u32 {
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

fn open_cell(
    board: &mut [Vec<Cell>],
    board_size: usize,
    row: usize,
    col: usize,
    state: &mut State,
) {
    if row >= board_size || col >= board_size {
        return;
    }

    let cell = &mut board[row][col];
    if cell.open {
        return;
    }

    cell.open = true;
    state.discovered_cells += 1;
    state.remaining_cells -= 1;

    if cell.neighbor > 0 {
        return;
    }

    if cell.contains_bomb {
        state.game_over = true;

        return;
    }

    if state.remaining_cells == 0 {
        state.game_over = true;

        return;
    }

    for r in -1isize..=1 {
        for c in -1isize..=1 {
            if r == 0 && c == 0 {
                continue;
            }

            let nr = row as isize + r;
            let nc = col as isize + c;

            if nr >= 0 && nr < board_size as isize && nc >= 0 && nc < board_size as isize {
                open_cell(board, board_size, nr as usize, nc as usize, state);
            }
        }
    }
}
