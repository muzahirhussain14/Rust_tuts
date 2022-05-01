// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum States {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl States {
    fn new (user_input: &str) -> Option<Self> {
        match user_input {
            "off" => Some(Self::Off),
            "sleep" => Some(Self::Sleep),
            "reboot" => Some(Self::Reboot),
            "shutdown" => Some(Self::Shutdown),
            "hibernate" => Some(Self::Hibernate),
            _ => None
        }
    }
}

fn get_input() -> io::Result<String> {

    println!("Please Enter any of the following options: ");
    println!("  - Off");
    println!("  - Sleep");
    println!("  - Reboot");
    println!("  - Shutdown");
    println!("  - Hibernate");
    
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn match_input(input: States) {

    match input {
        States::Off => println!("Off"),
        States::Sleep => println!("Sleeping"),
        States::Reboot => println!("Rebooting"),
        States::Shutdown => println!("Shutting down"),
        States::Hibernate => println!("Hibernating"),
    }
}

fn main() {

    let input = get_input();
    if input.is_ok() {
        let states = States::new(input.unwrap().to_lowercase().as_str());
        if states.is_some() {
            match_input(states.unwrap());
        }
        else {
            println!("Invalid Option Selected");
        }
    }
    else {
        println!("Invalid Input");
    }
}