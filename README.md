# Hangman game

![hangman](https://socialify.git.ci/krios2146/hangman/image?description=1&descriptionEditable=A%20simple%20hangman%20(word%20guessing)%20game%20with%20a%20console%20UI&font=Inter&language=1&name=1&owner=1&pattern=Solid&theme=Dark)

> https://en.wikipedia.org/wiki/Hangman_(game)

## Features

- Six mistakes are allowed
- Restart and exit by pressing `1` and `0`, respectively
- Dictionary of 1500+ English words, obtained
  from [this site](https://www.talkenglish.com/vocabulary/top-1500-nouns.aspx) using my own
  python [parser](https://github.com/krios2146/html-parser)
- I tried to make the user interface as friendly and intuitive as possible (a little bit of kaomoji)

## Run Locally

Clone the project

```bash
  git clone https://github.com/krios2146/hangman.git
```

Go to the project directory

```bash
  cd hangamn
```

Make sure you have installed Rust & Cargo - https://www.rust-lang.org/tools/install

```bash
  rustc --version
  cargo --version
```

Compile and run the program with Cargo

```bash
  cargo run
```

## Lessons Learned

It's my first Rust project, and I've never used it before. I wrote it immediately after reading the second chapter
of [The Rust Programming Language Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html). I don't
really get the "borrow" system, and the error handling is a little bit strange for me, but I enjoyed writing this
project
in Rust.

## Roadmap

- Infinite game mode
- Colorful output for mistake and success guesses
- Difficulty levels based on word length
