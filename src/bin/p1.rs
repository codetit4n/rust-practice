// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    inner: Vec<Bill>, //inner means we are taking the inner values of bills
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }

    fn get_all(&self) -> &Vec<Bill> {
        return &self.inner;
    }
}
fn get_input() -> String {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    buffer.trim().to_owned()
}

fn get_bill_amount() -> f64 {
    println!("Enter amount:");
    loop {
        let input: String = get_input(); //getting input
        let parsed_input: Result<f64, _> = input.parse(); //parsing input
        match parsed_input {
            Ok(amt) => return amt,
            Err(_) => println!("Please enter a number: "),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    //get the bill name
    println!("Enter bill name:");
    let name = get_input();
    //get the bill amount
    let amount = get_bill_amount();
    let bill = Bill { name, amount }; //since names and variables of struct are same we
                                      //don't need to specify. Compiler will figure it out
    bills.add(bill);
    println!("Bill added");
}

fn view_bill_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn main_menu() {
    fn show() {
        //function inside a function
        println!("");
        println!("=== Manage Bills ===");
        println!("1. Add bills");
        println!("2. View bills");
        println!("");
        println!("Enter selection:");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = get_input();
        match input.as_str() {
            "1" => add_bill_menu(&mut bills), //"1" automatically makes borrowed strings
            "2" => view_bill_menu(&bills),    //here its not going to modify bill so we borrow
            _ => break,
        }
    }
}

fn main() {
    main_menu();
}
