use rand::Rng;
use std::io;

#[derive(Debug)]
struct GuessInput {
    value: u32,
}

impl GuessInput {
    pub fn new(value: u32) -> GuessInput {
        if value < 1 {
            panic!("Guess value must be >= 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be <= 100, got {}.", value);
        }
        GuessInput { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn compare(&self, value: u32) -> i32 {
        if self.value == value {
            0
        } else if self.value > value {
            1
        } else {
            -1
        }
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess(between 1 and 100):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // validating user inputs an unsigned number not a string or any other type
        let guess: GuessInput = match guess.trim().parse() {
            Ok(num) => GuessInput::new(num),
            Err(_) => {
                println!("Please type an unsigned number!");
                continue;
            }
        };

        println!("You guessed: {}", guess.value());

        match guess.compare(secret_number) {
            -1 => println!("Too small!"),
            1 => println!("Too big!"),
            _ => {
                println!("You win!");
                break;
            }
        }
    }
}
