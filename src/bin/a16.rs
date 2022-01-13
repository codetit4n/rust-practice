// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student1 = Student {
        name: "Lokesh".to_owned(),
        locker: Some(2),
    };
    println!("name: {:?}", student1.name);
    match student1.locker {
        Some(locker_no) => println!("locker no: {:?}", locker_no),
        None => println!("no locker assigned"),
    }
}
