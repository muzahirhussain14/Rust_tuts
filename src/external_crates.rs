/*
Topic: External crates Crates.io
Requirements: Display the current date and time

Notes: 
    - Use the Chrono crate to work with time
    - Read the documentation section 'Formatting and Parsing' for example on how to create custom time formats
*/

use chrono::prelude::*;

fn main(){

    let utc: DateTime<Utc> = Utc::now();
    println!("Date: {}", utc.to_rfc2822());
}