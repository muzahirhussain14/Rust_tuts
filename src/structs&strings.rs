/*
Topic: Strings
Requirements:
    - Print out the name and favourite colors of the people aged 10 and under

Notes:
    - Use a struct for a personal age, name, and favourite color
    - The color and name should be store as a string
    - Create and store atleast 3 people in a vector
    - Iterate through the vector using a for..in loop
    - Use an if expression to determine which person's info should be printed
    - The name and color should be printed using a function
*/

struct PersonalInformation {
    // In structs we cannot use string literals. We have to use String.
    name: String,
    fav_color: String,
    age: u32
}

fn print(name: String, fav_color: String) {
    println!("\n{}'s favourite color is {}...\n", name, fav_color);
}

fn main() {
    let people = vec![
        PersonalInformation {
            name: "Muzahir".to_owned(),
            fav_color: String::from("Green"),
            age: 27
        },
        PersonalInformation {
            name: String::from("Kazim"),
            fav_color: String::from("Blue"),
            age: 25
        },
        PersonalInformation {
            name: "Qasim".to_owned(),
            fav_color: String::from("Red"),
            age: 9
        }
    ];

    for person in people {
        if person.age <= 10 {
            print(person.name, person.fav_color);
        }
    }
}