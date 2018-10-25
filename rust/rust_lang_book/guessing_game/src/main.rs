extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);
    guess_number(&secret_number);
}

fn guess_number(secret_number: &u32) {
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = parse_guess_to_number(&guess);
        
        if guess.eq(&0) {
            continue;
        }

        println!("You guessed: {}", guess);

        if compare_number(&guess, &secret_number) {
            break;
        }
    }
}

fn compare_number(actual: &u32, expected: &u32) -> bool {
    match actual.cmp(&expected) {
        Ordering::Less => {
            println!("Too small!");
            false
        },
        Ordering::Greater => {
            println!("Too big!");
            false
        },
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}

fn parse_guess_to_number(guess_str: &str) -> u32 {
    match guess_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            0
        }
    }
}
