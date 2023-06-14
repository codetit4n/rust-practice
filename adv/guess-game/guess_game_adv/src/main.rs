use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct GuessInput {
    value: u32,
}

impl GuessInput {
    fn value(&self) -> u32 {
        self.value
    }

    fn compare(&self, value: u32) -> Ordering {
        if self.value == value {
            Ordering::Equal
        } else if self.value > value {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: GuessInput = match guess.trim().parse() {
            Ok(num) => GuessInput { value: num },
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());

        match guess.compare(secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
