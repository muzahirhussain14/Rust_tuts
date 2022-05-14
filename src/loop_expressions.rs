/*
    Loop Expressions:
    Loop expressions allows us to return the value from the loop

    let value = 5
    let result: usize = 'ident: loop {
        break value;        // break and return the value to the result
        break ident;        // we can also return to a specific loop using loop label
    }
*/

use std::io;

fn main() {

    let mut buffer: String = String::new();

    let value: usize= loop {
        println!("Enter a number: ");
        let result = io::stdin().read_line(&mut buffer);

        if result.is_ok() {
            let user_input = buffer.trim().parse::<usize>();
            match user_input {
                    Ok(value) => break value,                               // break from the loop and return the value
                    Err(e) => {buffer.clear(); continue;},
            }
        }
    };
    println!("Value entered by the user is: {}", value);
}