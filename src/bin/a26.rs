// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats
use chrono::{DateTime, Local};

fn main() {
    let dt: DateTime<Local> = Local::now();
    println!("{:?}", dt);

    //printing in a formatted way: see documentation
    println!("{:?}", dt.format("%a %b %e %T %Y").to_string());
}
