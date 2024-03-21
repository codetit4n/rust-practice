// Convert temperatures between Fahrenheit and Celsius - From The Rust Book Chapter 3
use std::io;

fn main() {
    loop {
        println!("Menu:-");
        println!("1. Convert fahrenheit to celsius");
        println!("2. Convert celsius to fahrenheit");
        println!("0. Exit");

        let mut ch = String::new();

        io::stdin()
            .read_line(&mut ch)
            .expect("Failed to read line!");

        match ch.trim().parse::<i32>() {
            Ok(num) => match num {
                0 => break,
                1 => {
                    println!("Enter the temperature in fahrenheit:");
                    let mut temp = String::new();
                    io::stdin()
                        .read_line(&mut temp)
                        .expect("Failed to read line!");
                    let fahrenheit: f64 = temp.trim().parse().expect("Invalid temperature input!");
                    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                    println!("{}°F = {:.4}°C", fahrenheit, celsius);
                }
                2 => {
                    println!("Enter the temperature in celsius:");
                    let mut temp = String::new();
                    io::stdin()
                        .read_line(&mut temp)
                        .expect("Failed to read line!");

                    let celsius: f64 = temp.trim().parse().expect("Invalid temperature input!");
                    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
                    println!("{}°C = {}°F", celsius, fahrenheit);
                }
                _ => println!("Input should be between 0 and 2!!!"),
            },
            Err(_) => {
                println!("Please type an valid number between 0 and 2!!!");
                continue;
            }
        }
    }
}
