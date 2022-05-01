// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned

fn longest <'a> (a: &'a str, b: &'a str) -> &'a str {

    if a.len() >= b.len() {
        a
    }
    else {
        b
    }
}

fn main()
{
    let string1 = "Hello";
    let string2 = "worldd";

    println!("Longer String: {}", longest(&string1, &string2));
}