use myrustlib::{self, get_char};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct State {
    corrects: i32,
    misses: i32,
    won: bool,
    lose: bool,
    playing: bool,
}

fn main() {
    let mut state = State {
        corrects: 0,
        misses: 0,
        won: false,
        lose: false,
        playing: true,
    };
    let mut words: Vec<String> = Vec::new();
    load_file(&mut words, "input.txt");
    let selected_word: Vec<char> = select_word(&words).chars().collect();
    while state.playing {
        let guess = get_char("Type a letter to guess: ");
        println!("{:?}, char: {}", selected_word, guess);

        let mut correct_chars: [bool; 50] = [false; 50];
        build_hangman(&selected_word, guess, &mut correct_chars, &mut state);
        print_hangman(&selected_word, correct_chars, &state);
    }
}

fn load_file(words: &mut Vec<String>, infile: &str) {
    let file = match File::open(infile) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening the file: {e}");
            return;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("Error reading line: {e}");
                return;
            }
        };
        if line.is_empty() {
            continue;
        }

        println!("[ DEBUG ]: {line}");
        words.push(line);
    }
}

fn build_hangman(word: &[char], guess: char, correct_chars: &mut [bool; 50], state: &mut State) {
    let mut hit: bool = false;
    let mut remaining = word.len();
    for (i, &ch) in word.iter().enumerate() {
        if ch == guess {
            correct_chars[i] = true;
            hit = true;
            remaining -= 1;
        }
    }

    let mut corrects_count = 0;
    let mut misses_count = 0;
    if hit {
        corrects_count += 1;
    } else {
        misses_count += 1;
    }

    if remaining == 0 {
        state.won = true;
        state.playing = false;
        state.lose = false;
    }

    state.corrects = corrects_count;
    state.misses = misses_count;
}

fn print_hangman(word: &[char], correct_chars: [bool; 50], state: &State) {
    let stages = [
        "  +---+\n  |   |\n  |    \n  |    \n  |    \n  |    \n",
        "  +---+\n  |   |\n  |   O\n  |    \n  |    \n  |    \n",
        "  +---+\n  |   |\n  |   \x1b[1;33mO\x1b[0m\n  |   \x1b[1;33m|\x1b[0m\n  |    \n  |    \n",
        "  +---+\n  |   |\n  |   \x1b[1;33mO\x1b[0m\n  |   \x1b[1;33m|\\\x1b[0m\n  |    \n  |    \n",
        "  +---+\n  |   |\n  |   \x1b[1;33mO\x1b[0m\n  |  \x1b[1;33m/|\\\x1b[0m\n  |    \n  |    \n",
        "  +---+\n  |   |\n  |   \x1b[1;31mO\x1b[0m\n  |  \x1b[1;31m/|\\\x1b[0m\n  |    \x1b[1;31m\\\x1b[0m\n  |    \n",
        "  +---+\n  |   |\n  |   \x1b[1;31mO\x1b[0m\n  |  \x1b[1;31m/|\\\x1b[0m\n  |  \x1b[1;31m/ \\\x1b[0m\n  |    \n",
    ];
    let init_stage = state.misses;
    let stage = init_stage.clamp(0, 6);
    print!("{}", stages[stage as usize]);
    print!("  + ");
    for (i, &ch) in word.iter().enumerate() {
        if correct_chars[i] {
            print!("{}", ch)
        } else {
            print!("_")
        }
    }

    println!();
}

fn select_word(words: &[String]) -> String {
    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(0..(words.len() - 1));
    let selected_word = &words[rand_number];
    selected_word
        .chars()
        .map(|_| '_')
        .for_each(|c| print!("{}", c));
    println!();
    println!("Random: {}", selected_word);

    selected_word.clone()
}
