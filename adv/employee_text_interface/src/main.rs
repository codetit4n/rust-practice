use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};

#[derive(Debug)]
enum Choice {
    Exit,
    Add,
    DepartmentPeople,
    SortedList,
}

impl Choice {
    fn choice(i: u32) -> Option<Choice> {
        match i {
            0 => Some(Choice::Exit),
            1 => Some(Choice::Add),
            2 => Some(Choice::DepartmentPeople),
            3 => Some(Choice::SortedList),
            _ => None,
        }
    }
}

fn main() {
    let mut department_to_names = HashMap::<String, Vec<String>>::new();
    let mut name_to_department = HashMap::<String, String>::new();
    loop {
        let mut choice = String::new();
        clear_screen();
        show_menu();

        stdin()
            .read_line(&mut choice)
            .expect("Failed to read from stdin");

        choice = choice.trim().to_string();

        match choice.parse::<u32>() {
            Ok(i) => match Choice::choice(i) {
                Some(ch) => match ch {
                    Choice::Exit => break,
                    Choice::Add => {
                        clear_screen();
                        println!("Input in format 'Add [Employee] to [Department]':");
                        let mut input = String::new();
                        stdin()
                            .read_line(&mut input)
                            .expect("Failed to read from stdin");
                        input = input.trim().to_string();
                        let words: Vec<&str> = input.split_whitespace().collect();
                        match words.as_slice() {
                            ["Add", name, "to", department] => {
                                add_name_to_department(
                                    &mut department_to_names,
                                    &mut name_to_department,
                                    department,
                                    name,
                                );
                            }
                            ["add", name, "to", department] => {
                                add_name_to_department(
                                    &mut department_to_names,
                                    &mut name_to_department,
                                    department,
                                    name,
                                );
                            }
                            _ => {
                                invalid_input_err();
                                continue;
                            }
                        }
                    }
                    Choice::DepartmentPeople => {
                        clear_screen();
                        println!("Enter department:");
                        let mut input = String::new();
                        stdin()
                            .read_line(&mut input)
                            .expect("Failed to read from stdin");
                        input = input.trim().to_string();
                        let names: Vec<String> = if let Some(vec) = department_to_names.get(&input)
                        {
                            vec.to_vec()
                        } else {
                            println!("No employees in this department yet!");
                            pause();
                            continue;
                        };
                        println!("List of employees in this department: {:?}", names);
                        pause();
                    }
                    Choice::SortedList => {
                        clear_screen();
                        let mut to_show: Vec<String> = Vec::new();
                        for (k, v) in &name_to_department {
                            to_show.push(format!("{k} at {v}"));
                        }
                        if to_show.len() > 0 {
                            to_show.sort();
                            for val in &to_show {
                                println!("{val}")
                            }
                        } else {
                            println!("No employees found!")
                        }
                        pause();
                    }
                },
                None => {
                    invalid_input_err();
                    continue;
                }
            },
            Err(_) => {
                invalid_input_err();
                continue;
            }
        };
    }
}

fn add_name_to_department(
    department_to_names: &mut HashMap<String, Vec<String>>,
    name_to_department: &mut HashMap<String, String>,
    department: &str,
    name: &str,
) {
    let mut names: Vec<String> = if let Some(vec) = department_to_names.get(department) {
        vec.to_vec()
    } else {
        Vec::new()
    };
    names.push(name.to_string());
    name_to_department.insert(name.to_string(), department.to_string());
    department_to_names.insert(department.to_string(), names);
    println!("Successfully added {name} to {department}!");
    pause();
}

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();

    println!("-----------------------------------");
}

fn invalid_input_err() {
    println!("Invalid input!!!");
    pause();
}

fn show_menu() {
    println!("Menu:");
    println!("1. Add new employee");
    println!("2. List all employees in a department");
    println!("3. List all employees in the company by department, sorted alphabetically");
    println!("0. Exit");

    println!("-----------------------------------");
}
