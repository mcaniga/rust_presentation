use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

const INTRO_MSG: &str = "Guess the number!";
const PROMPT_MSG: &str = "Please input your guess.";
const READ_LINE_ERR_MSG: &str = "Please input your guess.";

#[derive(strum_macros::Display)]
enum GuessResult {
    TooSmall,
    TooBig,
    YouWon,
}

fn guess_the_number() {
    println!("{}", INTRO_MSG);
    let secret_number = rand::thread_rng().gen_range(1..101);
    // secret_number = 6; // this would throw [E0384]
    loop {
        println!("{}", PROMPT_MSG);
        let mut guess = String::new();
        // load line to 'guess' var
        io::stdin().read_line(&mut guess).expect(READ_LINE_ERR_MSG);
        // convert 'guess' to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        // compare with secret_number and do appropriate action
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", GuessResult::TooSmall),
            Ordering::Greater => println!("{}", GuessResult::TooBig),
            Ordering::Equal => {
                println!("{}", GuessResult::YouWon);
                break;
            }
        }
    }
}

pub fn run_basics_demo() {
    guess_the_number();
}
