/*
    Equality and ordering
*/


// PartialEq is used to check the equality while PartialOrd is used for the comparison.
#[derive(PartialEq, PartialOrd)]  
enum Floor {
    ClientServices,
    Marketing,
    Ops,
}


#[derive(PartialEq, PartialOrd)]  
struct User {
    id: u32,
    name: String,
}

fn main() {

    // ********************* ENUMs *********************
    // example of equal/unequal enum values
    let f1 = Floor::ClientServices;
    let f2 = Floor::Marketing;

    if f1 == f2 {
        println!("Same Floors");
    }
    else {
        println!("Different Floors");
    }

    // example of greater/less enum values

    if f1 > f2 {
        println!("Greater")
    }
    else {
        println!("Smaller")
    }




    // ********************* STRUCTs *********************
    let u1 = User {id: 16, name: "Kazim".to_owned()};
    let u2 = User {id: 17, name: "Kazim".to_owned()};
    
    if u1 == u2 {
        println!("Same Users");
    }
    else {
        println!("Different Users");
    }

    // compares each fields of the structs and stops if any field is unequal and return the result, else it will result in equal.
    // if we are looking for different behaviour we can always implement these methods and override the default behaviour
    if u1 > u2 {
        println!("User 1 is greater than User 2");
    }
    else if u1 < u2 {
        println!("User 1 is less than User 2")
    }
    else{
        println!("User 1 and User 2 both have same values");
    }
}