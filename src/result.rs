/*
Topic: Result

Requirements:
    - Determine if a customer is able to make a restricted purchase
    - Restricted purchases require that the age of the customers is atleast 21

Notes:
    - Use a struct to store atleast the age of the customer
    - Use a function to determine if a customer can make a restricted purchase
    - Return a result from the function
    - The Err variant should detail the reason why they cannot make a purchase
*/

struct Customer {
    age: u32
}

fn check_purchase(customer: Customer) -> Result<(), String>{

    if customer.age >= 21 {
        return Ok(())
    }

    Err("This customer cannot make a restricted purchase".to_owned())
}

fn main() {

    let customer_1: Customer = Customer { 
        age: 22
    };

    let customer_2: Customer = Customer {
        age: 19
    };
    
    match check_purchase(customer_1) {
        Ok(_) => println!("Customer 1: Purchase successful"),
        Err(msg) => println!("Customer 1: {}", msg)
    };

    match check_purchase(customer_2) {
        Ok(_) => println!("Customer 2: Purchase successful"),
        Err(msg) => println!("Customer 2: {}", msg)
    };
}