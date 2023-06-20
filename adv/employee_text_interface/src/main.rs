// INCOMPLETE
use std::{collections::HashMap, io};

fn main() {
    let mut department_to_names = HashMap::<String, Vec<String>>::new();
    loop {
        let mut choice = String::new();

        println!("-----------------------------------");

        println!("Choose:");
        println!("1. Add [Employee] to [department]");
        println!("2. List all people in a department");
        println!("3. List all people in the company by department, sorted alphabetically");
        println!("0. Exit");

        println!("-----------------------------------");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read from stdin");

        println!("-----------------------------------");

        choice = choice.trim().to_string();

        let ch = match choice.parse::<u32>() {
            Ok(i) => {
                if i <= 3 {
                    i
                } else {
                    println!("Invalid input!!!");
                    continue;
                }
            }
            Err(_) => {
                println!("Invalid input!!!");
                continue;
            }
        };

        match ch {
            0_u32 => break,
            1_u32 => {
                // @todo
                /*println!("Input in format: Add [Employee] to [Department]");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");
                input = input.trim().to_string()*/
                add_name_to_department(&mut department_to_names, "sales", "baban");
                add_name_to_department(&mut department_to_names, "sales", "ankit");
                add_name_to_department(&mut department_to_names, "sales", "zerard");
            }
            2_u32 => {
                println!("Enter department:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");
                input = input.trim().to_string();
                let names: Vec<String> = if let Some(vec) = department_to_names.get(&input) {
                    vec.to_vec()
                } else {
                    println!("No employees in this department yet!");
                    continue;
                };
                println!("List of employees in this department: {:?}", names);
            }
            3_u32 => println!("three"),
            _ => (),
        }
    }
}

fn add_name_to_department(map: &mut HashMap<String, Vec<String>>, department: &str, name: &str) {
    let mut names: Vec<String> = if let Some(vec) = map.get(department) {
        vec.to_vec()
    } else {
        Vec::new()
    };
    names.push(name.to_string());
    names.sort();
    map.insert(department.to_string(), names);
    println!("{:?}", map);
}
