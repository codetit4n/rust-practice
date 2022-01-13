// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
    dimensions: Dimentions,
    weight: f64,
    color: BoxColor,
}

struct Dimentions {
    length: f64,
    width: f64,
    height: f64,
}

impl Dimentions {
    fn print(&self) {
        println!("length: {:?}", self.length);
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
    }
}

enum BoxColor {
    Red,
    Blue,
}
impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Red => println!("red"),
            BoxColor::Blue => println!("blue"),
        }
    }
}

impl Box {
    fn new_box(weight: f64, color: BoxColor, dimensions: Dimentions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_characterstics(&self) {
        println!("weight: {:?}", self.weight);
        self.dimensions.print();
        self.color.print();
    }
}

fn main() {
    let box1 = Box {
        dimensions: Dimentions {
            length: 2.3,
            width: 4.1,
            height: 6.0,
        },
        weight: 44.2,
        color: BoxColor::Red,
    };
    box1.print_characterstics();
    println!("----------------------------------");

    let box2 = Box::new_box(
        23.0,
        BoxColor::Blue,
        Dimentions {
            length: 3.4,
            width: 2.1,
            height: 5.0,
        },
    );
    box2.print_characterstics();
}
