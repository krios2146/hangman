use std::io;

fn main() {
    let word = choose_word_for_game();

    let word_size = word.chars().count();
    let mut word_mask = "*".repeat(word_size);

    let mut mistakes_counter = 0;
    loop {
        print_hangman(mistakes_counter);
        println!("{word_mask}");

        let guess = get_user_guess();

        if word.contains(&guess) {
            word_mask = update_word_mask(&word, &word_mask, &guess);
        } else {
            mistakes_counter += 1;
        }

        if mistakes_counter == 5 {
            println!("You lost");
            break;
        }
        if !word_mask.contains("*") {
            println!("You win");
            break;
        }
    }
}

// TODO: should return random word from /resources/dictionary.txt
fn choose_word_for_game() -> String {
    "world".to_string()
}

// TODO: should print ASCII hangman
fn print_hangman(mistakes: i32) {
    println!("|There is a hangman|")
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
