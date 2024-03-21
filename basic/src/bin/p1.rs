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

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    inner: HashMap<String, Bill>, //inner means we are taking the inner values of bills
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
        //reason to clone is because the key
        //needs to be a owned string and bill name also need to be a owned string
    }

    fn get_all(&self) -> Vec<Bill> {
        //here we will build a vector from hashmap
        let mut bills = vec![];
        for bill in self.inner.values() {
            //here we are only interested in values
            bills.push(bill.clone());
        }
        //note when iterating through hashmaps bill in for loop is always borrowed
        //and we don't want to push in a borrowed item so we clone
        bills //note here we are returning a owned vector not borrowed so after this
              //function is done executing it will transfer the ownership of this
              //vector to wherever it is called
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some() //is_some() returns true if option is some
                                          //value
    }
    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            //get_mut returns an optional mutable value
            //it allows us to mutate the value
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Enter amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse(); //parsing input
        match parsed_input {
            Ok(amt) => return Some(amt),
            Err(_) => println!("Please enter a number: "),
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    //get the bill name
    println!("Enter bill name:");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    //get the bill amount
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    let bill = Bill { name, amount };
    //since names and variables of struct are same we
    //don't need to specify. Compiler will figure it out
    bills.add(bill);
    println!("Bill added");
}

fn remove_bill_menu(bills: &mut Bills) {
    //first we need to view the bills we are deleting
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to remove:");
    //input
    let input = match get_input() {
        Some(input) => input,
        None => return,
    };
    if bills.remove(&input) {
        println!("removed");
    } else {
        println!("bill not found!");
    }
}

fn update_bill_menu(bills: &mut Bills) {
    //first we need to view the bills we are deleting
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to update:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    if bills.update(&name, amount) {
        println!("updated")
    } else {
        println!("bill not found!")
    }
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
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("Enter nothing to exit");
        println!("");
        println!("Enter selection:");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills), //"1" automatically makes borrowed strings
            "2" => view_bill_menu(&bills),    //here its not going to modify bill so we borrow
            "3" => remove_bill_menu(&mut bills),
            "4" => update_bill_menu(&mut bills),
            _ => break,
        }
    }
}

fn main() {
    main_menu();
}
