// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let ticket1 = Ticket::Backstage(600.0, "Lokesh".to_owned());
    let ticket2 = Ticket::Vip(500.0, "Lohit".to_owned());
    let ticket3 = Ticket::Standard(100.0);

    let tickets = vec![ticket1, ticket2, ticket3];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, name) => {
                println!("Backstage ticket price: {:?}, name: {:?}", price, name)
            }
            Ticket::Vip(price, name) => println!("VIP ticket price: {:?}, name: {:?}", price, name),
            Ticket::Standard(price) => println!("Standard ticket price: {:?}", price),
        }
    }
}
