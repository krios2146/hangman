use std::fs::File;
use std::io;
use std::io::BufRead;

use rand::Rng;

fn main() {
    const MAX_MISTAKES: i32 = 6;

    let word = choose_word_for_game();
    let word_size = word.chars().count();

    let mut word_mask = "*".repeat(word_size);

    let mut mistakes_counter = 0;
    loop {
        print_hangman(mistakes_counter);
        println!("Word:     {word_mask}");
        println!("Mistakes: {mistakes_counter}");

        if mistakes_counter == MAX_MISTAKES {
            println!("You lost");
            break;
        }
        if !word_mask.contains("*") {
            println!("You win");
            break;
        }

        let guess = get_user_guess();

        if word.contains(&guess) {
            word_mask = update_word_mask(&word, &word_mask, &guess);
        } else {
            mistakes_counter += 1;
        }
    }
}

fn choose_word_for_game() -> String {
    let file = File::open("resources/dictionary.txt").expect("Cannot open dictionary.txt");
    let reader = io::BufReader::new(file);

    let mut words = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            words.push(line)
        }
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..words.len());

    let random_word = &words[random_index];
    return random_word.to_string();
}

fn print_hangman(mistakes: i32) {
    let padding = "         ";
    // top
    println!("{}----------", padding);
    println!("{}|        |", padding);

    // hangman
    match mistakes {
        0 => {
            println!("{}|        ", padding);
            println!("{}|        ", padding);
            println!("{}|        ", padding);
        }
        1 => {
            println!("{}|        O", padding);
            println!("{}|        ", padding);
            println!("{}|        ", padding);
        }
        2 => {
            println!("{}|        O", padding);
            println!("{}|        |", padding);
            println!("{}|        ", padding);
        }
        3 => {
            println!("{}|        O", padding);
            println!("{}|       -|", padding);
            println!("{}|        ", padding);
        }
        4 => {
            println!("{}|        O", padding);
            println!("{}|       -|-", padding);
            println!("{}|        ", padding);
        }
        5 => {
            println!("{}|        O", padding);
            println!("{}|       -|-", padding);
            println!("{}|       / ", padding);
        }
        _ => {
            println!("{}|        O", padding);
            println!("{}|       -|-", padding);
            println!("{}|       / \\", padding);
        }
    }

    // bot
    println!("{}|         ", padding);
    println!("{}----------", padding);
    println!("( ﾉ ﾟｰﾟ)ﾉ  {} ＼(ﾟｰﾟ＼))", padding);
}

fn get_user_guess() -> String {
    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim_end();

                if trimmed_input.len() == 1 {
                    return trimmed_input.to_string();
                } else {
                    println!("Please enter exactly one letter");
                }
            }
            Err(error) => {
                eprintln!("Failed to read input: {error}")
            }
        }
    }
}

fn update_word_mask(word: &String, word_mask: &String, guess: &String) -> String {
    let indices_replacements: Vec<_> = word.match_indices(guess).collect();
    let mut word_mask: Vec<char> = word_mask.chars().collect();

    for (index, replacement) in indices_replacements {
        if let Some(char_at_index) = word_mask.get_mut(index) {
            *char_at_index = replacement.parse().unwrap();
        }
    }

    return word_mask.into_iter().collect();
}
