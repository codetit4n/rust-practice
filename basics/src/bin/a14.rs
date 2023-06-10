// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 21 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    fav_col: String,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}
fn print_col(col: &str) {
    println!("favorite color: {:?}", col);
}

fn main() {
    let people = vec![
        Person {
            name: "Lokesh".to_owned(),
            age: 21,
            fav_col: "Blue".to_owned(),
        },
        Person {
            name: "Lohit".to_owned(),
            age: 21,
            fav_col: "Orange".to_owned(),
        },
        Person {
            name: "Swarinim".to_owned(),
            age: 22,
            fav_col: "Purple".to_owned(),
        },
    ];

    for person in people {
        if person.age <= 21 {
            print_name(&person.name);
            print_col(&person.fav_col);
        }
    }
}
