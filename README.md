# Guessing Game

A two-player command-line guessing game written in Rust. Both players compete to guess a randomly generated number between 1 and 100.

## Features

- Two-player gameplay
- Random number generation using the `rand` crate
- 5 guesses per player
- Real-time feedback (Too small/Too big)
- Input validation with error handling

## How to Play

1. Run the game
2. Player One enters their guess
3. Player Two enters their guess
4. Both players receive feedback on their guesses (too small, too big, or correct)
5. The game continues until either player guesses the correct number or all 5 guesses are used up
6. The first player to guess correctly wins!

## Building and Running

### Prerequisites

- Rust toolchain (1.56 or later)
- Cargo

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

## Project Structure

```
.
├── Cargo.toml       # Project manifest and dependencies
├── src/
│   └── main.rs      # Game logic and entry point
└── README.md        # This file
```

## Dependencies

- `rand = "0.8.5"` - For generating random numbers

## Example Gameplay

```
Guess the number!
Player One Please input your guess.
50
Player Two Please input your guess.
75
Player One guessed: 50
Player One Too small!
Player Two guessed: 75
Player Two Too big!
Guess Left 4
```
