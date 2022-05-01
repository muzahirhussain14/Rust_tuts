
// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculate_perimeter(&self) -> u32;
}

struct Square {
    side: u32
}
impl Perimeter for Square {
    fn calculate_perimeter(&self) -> u32 {
        self.side * 4
    }
}


struct Triangle {
    a: u32,
    b: u32,
    c: u32
}
impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> u32 {
        self.a * self.b * self.c
    }
}

fn print_perimeter(shape: impl Perimeter) {

    let perimeter = shape.calculate_perimeter();
    println!("Perimeter: {}", perimeter);
}

fn main() {
     
    let square = Square {
        side: 10,
    };

    let triangle = Triangle { a: 5, b:10, c:2 };

    print_perimeter(square);
    print_perimeter(triangle);
}