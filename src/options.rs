/*
Topic: Options

Requirements:
    - Print out the details of the students' locker assignment
    - Lockers use numbers and are optional for students

Notes:
    - Use a structure containing the student's name and locker assignment
    - The locker assignment should use an Option<i32>
*/

struct StudentLocker {
    name: String,
    locker_no: Option<u32>
}

fn main() {

    let locker = StudentLocker {
        name: "Muzahir Hussain".to_owned(),
        locker_no: Some(121)
    };

    // printing the details
    println!("Name: {}", locker.name);
    match locker.locker_no {
        Some(n) => println!("Locker number: {}", n),
        None => println!("Locker not assigned")
    }
}