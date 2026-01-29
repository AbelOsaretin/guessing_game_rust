use std::cmp::Ordering;

use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter = 0;
    let mut guess_tries = 5;
    // println!("The secret number is: {secret_number}");

    loop {
        if counter < guess_tries {
            // println!("The secret number is: {secret_number}");

            println!("Player One Please input your guess.");

            let mut player_one_guess = String::new();
            let mut player_two_guess = String::new();

            io::stdin()
                .read_line(&mut player_one_guess)
                .expect("Failed to read line");

            println!("Player Two Please input your guess.");

            io::stdin()
                .read_line(&mut player_two_guess)
                .expect("Failed to read line");
            let player_one_guess: u32 = match player_one_guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let player_two_guess: u32 = match player_two_guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("Player One guessed: {player_one_guess}");
            println!("Player Two guessed: {player_two_guess}");

            match player_one_guess.cmp(&secret_number) {
                Ordering::Less => println!("Player OneToo small!"),
                Ordering::Greater => println!("Player One Too big!"),
                Ordering::Equal => {
                    println!("Player One You win!");
                    break;
                }
            }

            match player_two_guess.cmp(&secret_number) {
                Ordering::Less => println!("Player Two Too small!"),
                Ordering::Greater => println!("Player Two Too big!"),
                Ordering::Equal => {
                    println!("Player Two You win!");
                    break;
                }
            }
            counter += 1;

            println!("Guess Left {}", guess_tries - counter);
        } else {
            println!("You have used up your guess");
            break;
        }
    }
}
