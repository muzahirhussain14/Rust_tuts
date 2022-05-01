/*
Reference Counter --> Smart Pointer
*/

use std::rc::Rc;

#[derive(Debug)]
struct Vehicle {
    id: String
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Vehicle>,
}

fn main() {

    let mut car = Rc::new(Vehicle {id: "3411".to_owned()});

    let door_1 = Door {vehicle: Rc::clone(&car)};
    let door_2 = Door {vehicle: Rc::clone(&car)};

    drop(car);

    println!("Door 1: {:?}", door_1);
    println!("Door 2: {:?}", door_2);
}