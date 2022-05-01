/*
Topic: Vectors
Requirements:
    - Print 10, 20, "thirty", and 40 in a loop
    - Print total number of elements in a vector

Notes:
    - Use a vector to store 4 numbers
    - Iterate through the vector using a for..in loop
    - Determine whether to print the number or print "thirty" inside the for loop
    - Use the .len() to print the number of elements in a vector at the end.
*/

fn main() {

    let my_numbers = vec![10,20,30,40];
    for number in &my_numbers {
        match number {
            30 => println!("thirty"),
            _ => println!("{}", number)
        }
    }

    println!("Length: {}", my_numbers.len());
}