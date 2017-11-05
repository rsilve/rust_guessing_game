extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

const SECRET_UPPER_LIMIT: usize = 101;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1, SECRET_UPPER_LIMIT);
    println!("The secret number is: {}", secret);

    loop {
        match guess_secret(&secret) {
            Ok(result) => {
                println!("{}", result);
                break;
            }
            Err(GameError::NotInRange) => {
                println!("The guess is invalid (must be between 1 and 100");
                continue;
            }
            Err(GameError::Parse(e)) => {
                println!("Cannot read the guess: {}", e);
                continue;
            }
        }
    }
}

fn guess_secret(secret: &usize) -> Result<&str, GameError> {
    let guess = read_gess()?;
    let guess = validate_guess(guess)?;
    match guess.cmp(secret) {
        Ordering::Less => Ok("Too small!"),
        Ordering::Greater => Ok("Too big!"),
        Ordering::Equal => Ok("You win!")
    }
}

fn read_gess() -> Result<usize, GameError> {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line !!!");

    guess.trim().parse::<usize>().map_err(|err| {
        GameError::Parse(err)
    })
}


fn validate_guess(value: usize) -> Result<usize, GameError> {
    if value < 1 || value >= SECRET_UPPER_LIMIT {
        Err(GameError::NotInRange)
    } else {
        Ok(value)
    }
}


#[derive(Debug)]
pub enum GameError {
    NotInRange,
    Parse(std::num::ParseIntError)
}
