use myrustlib::{self, get_char, get_string};
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct State {
    greetings: bool,
    corrects: i32,
    misses: i32,
    correct_chars: String,
    missed_chars: String,
    remaining: usize,
    won: bool,
    lose: bool,
    playing: bool,
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        println!("Usage: {} <dictionary file>", argv[0]);
        return;
    }

    let mut words: Vec<String> = Vec::new();
    load_file(&mut words, &argv[1]);
    loop {
        let opt_selected_word = select_word(&words);
        let selected_word: Vec<char> = match opt_selected_word {
            Some(w) => w.chars().collect(),
            None => panic!("Without the file, this program can't run anymore."),
        };
        let mut correct_chars: Vec<bool> = vec![false; selected_word.len()];
        let mut state = State {
            greetings: true,
            corrects: 0,
            misses: 0,
            correct_chars: String::new(),
            missed_chars: String::new(),
            remaining: selected_word.len(),
            won: false,
            lose: false,
            playing: true,
        };
        while state.playing {
            print_hangman(&selected_word, &correct_chars, &state);
            state.greetings = false;
            let guess =
                get_string("Type a letter to guess how to free de Hangman: ").to_uppercase();
            for ch in guess.chars() {
                build_hangman(&selected_word, ch, &mut correct_chars, &mut state);
            }
        }

        if !state.playing {
            print_hangman(&selected_word, &correct_chars, &state);
            let option = get_char("Do you would like to play again? (y/n): ");
            if option == 'n' {
                return;
            }
        }
    }
}

fn load_file(words: &mut Vec<String>, infile: &str) {
    let file = match File::open(infile) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening the file {infile}: {e}");
            return;
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l.to_uppercase(),
            Err(e) => {
                println!("Error reading line: {e}");
                return;
            }
        };
        if line.is_empty() {
            continue;
        }

        words.push(line);
    }
}

fn build_hangman(word: &[char], guess: char, correct_chars: &mut [bool], state: &mut State) {
    let mut hit: bool = false;
    for (i, &ch) in word.iter().enumerate() {
        if (ch == guess || ch == '-') && !correct_chars[i] {
            correct_chars[i] = true;
            hit = true;
            state.remaining -= 1;
        }
    }

    if hit {
        state.corrects += 1;
        state.correct_chars.push(guess);
    } else {
        state.misses += 1;
        state.missed_chars.push(guess);
    }

    if state.misses >= 6 {
        state.lose = true;
        state.playing = false;
        state.won = false;
    }

    if state.remaining == 0 {
        state.won = true;
        state.playing = false;
        state.lose = false;
    }
}

fn print_hangman(word: &[char], correct_chars: &[bool], state: &State) {
    println!("\x1b[2J\x1b[1;1H");
    if state.greetings {
        println!(
            r"
     /$$   /$$                                                                
    | $$  | $$                                                                
    | $$  | $$  /$$$$$$  /$$$$$$$   /$$$$$$  /$$$$$$/$$$$   /$$$$$$  /$$$$$$$ 
    | $$$$$$$$ |____  $$| $$__  $$ /$$__  $$| $$_  $$_  $$ |____  $$| $$__  $$
    | $$__  $$  /$$$$$$$| $$  \ $$| $$  \ $$| $$ \ $$ \ $$  /$$$$$$$| $$  \ $$
    | $$  | $$ /$$__  $$| $$  | $$| $$  | $$| $$ | $$ | $$ /$$__  $$| $$  | $$
    | $$  | $$|  $$$$$$$| $$  | $$|  $$$$$$$| $$ | $$ | $$|  $$$$$$$| $$  | $$
    |__/  |__/ \_______/|__/  |__/ \____  $$|__/ |__/ |__/ \_______/|__/  |__/
                                   /$$  \ $$                                  
                                  |  $$$$$$/                                  
                                   \______/                                   
        "
        );
    }
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
        if correct_chars[i] || ch == '-' {
            print!("{}", ch)
        } else {
            print!("_")
        }
    }

    println!("\n");
    println!(
        "\x1b[1;37mCorrect chars: \x1b[32m{}\x1b[0m\t\
        \x1b[1;37mMissed chars: \x1b[31m{}\x1b[0m\n",
        state.correct_chars, state.missed_chars
    );
    if state.won {
        println!("\x1b[1;36mYYYEEAHHHH!!! You've saved the hangman!\x1b[0m\n");
    }

    if state.lose {
        println!(
            "\x1b[1;31mYou've killed the hangman!\x1b[0m The word was: \x1b[1m{}\x1b[0m\n",
            word.iter().collect::<String>()
        );
    }
}

fn select_word(words: &[String]) -> Option<String> {
    if words.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    let rand_number = rng.gen_range(0..(words.len()));
    let selected_word = &words[rand_number];

    Some(selected_word.clone())
}
