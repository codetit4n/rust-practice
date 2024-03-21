// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Green,
    Blue,
}
fn show_color_name(color: Colors) {
    match color {
        Colors::Red => println!("Red!"),
        Colors::Green => println!("Green!"),
        Colors::Blue => println!("Blue!"),
    }
}
fn main() {
    let green = Colors::Green;
    let red = Colors::Red;
    let blue = Colors::Blue;
    show_color_name(green);
    show_color_name(red);
    show_color_name(blue);
}
