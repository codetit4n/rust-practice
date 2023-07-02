use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // Note: Using unwrap_or_else allows us to define some custom, non-panic! error handling.

    if let Err(err) = minigrep::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}
