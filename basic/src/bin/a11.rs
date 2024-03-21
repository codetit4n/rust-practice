// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    id: i32,
    qty: i32,
}

fn display_id(item: &Item) {
    println!("id: {:?}", item.id);
}

fn display_qty(item: &Item) {
    println!("qty: {:?}", item.qty);
}

fn main() {
    let itm = Item { id: 5, qty: 9 };
    display_id(&itm);
    display_qty(&itm);
}
