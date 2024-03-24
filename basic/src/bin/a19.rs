// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);

    let mut ctr = 0;
    for (item, qty) in stock.iter() {
        ctr += qty;
        let nos = if qty == &0 {
            "out of stock!!!".to_owned()
        } else {
            format!("{:?}", qty)
            //similar to println macro instead of printring the line on the console
            //its going to put it into a string.
        };
        println!("Item: {:?}, Stock: {:?}", item, nos);
    }
    println!("Total number of items in stock: {:?}", ctr);
}
