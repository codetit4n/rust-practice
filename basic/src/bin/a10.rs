// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
fn print_msg(my_bool: bool) {
    if my_bool {
        println!("its big")
    } else {
        println!("its small")
    }
}

fn main() {
    let val = 500;
    let my_bool = val > 100;
    // let my_bool = if val > 100 { true } else { false };
    print_msg(my_bool);
}
