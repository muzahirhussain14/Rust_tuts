/*
Topic: Advanced Match

Requirements:
    - Printout a list of tickets and their information for an event
    - Tickets can be Backstage, VIP and Standard
    - Backstage and VIP tickets includes ticket holder's name
    - All tickets include the price

Notes:
    - Use an enum for tickets with data associated with each variant
    - Create one of each ticket and place it into a vector
    - Use a match expression while iterating the vector to print the ticket information.
*/

enum Ticket{
    Backstage(u32, String),
    VIP(u32, String),
    Standard(u32)
}

fn main() {

    let tickets = vec![
        Ticket::Backstage(17, "Ali".to_owned()),
        Ticket::VIP(20, "Muzahir".to_owned()),
        Ticket::Standard(12)
    ];

    for ticket in &tickets {
        match ticket {
            Ticket::Backstage(amount, holder) => println!("Backstage Ticket -- Amount: {}, Holder: {}", amount, holder),
            Ticket::VIP(amount, holder) => println!("VIP Ticket -- Amount: {}, Holder: {}", amount, holder),
            Ticket::Standard(amount) => println!("Standard Ticket -- Amount: {}", amount)
        }
    }
}