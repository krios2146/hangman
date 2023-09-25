use std::io;

fn main() {
    let word = choose_word_for_game();

    let word_size = word.chars().count();
    let mut word_mask = "*".repeat(word_size);

    println!("{word_mask}");

    let guess = get_user_guess();

    // TODO: check if guess is correct & either update word_mask or mistakes counter

    print_hangman()
}

// TODO: should print ASCII hangman
fn print_hangman() {
    println!("|There is a hangman|")
}

// TODO: should return random word from /resources/dictionary.txt
fn choose_word_for_game() -> String {
    "world".to_string()
}

// TODO: add validation & `panic!` doesn't look correct
fn get_user_guess() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(error) => panic!("Failed to read input: {error}"),
    }
}
