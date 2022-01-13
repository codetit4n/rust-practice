// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavours {
    Orange,
    Guava,
    Mango,
}

struct Fluid {
    flavour: Flavours,
    fluid_ounce: f64,
}

fn print_flavours_ounces(fluid: Fluid) {
    match fluid.flavour {
        Flavours::Orange => println!("flavour: orange"),
        Flavours::Guava => println!("flavour: guava"),
        Flavours::Mango => println!("flavour: mango"),
    }
    println!("ounce: {:?}", fluid.fluid_ounce);
}
fn main() {
    let fld1 = Fluid {
        flavour: Flavours::Orange,
        fluid_ounce: 3.0,
    };
    let fld2 = Fluid {
        flavour: Flavours::Guava,
        fluid_ounce: 4.3,
    };
    let fld3 = Fluid {
        flavour: Flavours::Mango,
        fluid_ounce: 8.1,
    };
    print_flavours_ounces(fld1);
    println!("--------------------------");
    print_flavours_ounces(fld2);
    println!("--------------------------");
    print_flavours_ounces(fld3);
}
