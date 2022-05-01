
/*
    Operator Overloading
*/
use std::ops::Add;

struct Number(i32);

impl Add<i32> for Number {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {            // overload Add operator for the Number struct
        Number(self.0 + rhs)
    }
}

fn main() {

    let n1 = Number(55);
    let n2 = n1 + 5;

    println!("N2: {}", n2.0);
}