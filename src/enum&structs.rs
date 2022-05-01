/*
Topic: Implementing functionality with the impl keyword
Requirements:
    - Prints the characteristics of the shipping box
    - Must include dimentions, weight and color
    
Notes:
    - Use a struct to encapsulate box characteristics
    - Use an enum for box color
    - Implement a functionality on the box struct to create a new box
    - Implement a functionality on the box struct to print the characteristics
*/

enum Color {
    Red,
    Blue,
    Green
}

struct Dimentions {
    length: u32,
    width: u32,
    height: u32
}

impl Dimentions {
    fn new(l: u32, w: u32, h: u32) -> Self {
        Self {
            length: l,
            width: w,
            height: h
        }
    }
}

struct ShippingBox {
    color: Color,
    dimentions: Dimentions,
    weight: u32
}

impl ShippingBox {
    fn new (color: Color, dimentions: Dimentions, weight: u32) -> Self {
        Self {
            // uses short-hand initialization syntax, because the names of the variables are same as defined in the structs
            color,
            dimentions,
            weight
        }
    }

    fn print(&self) {
        println!("Characteristics of the Box");
        match self.color {
            Color::Red => println!("Color: Red"),
            Color::Green => println!("Color: Green"),
            Color::Blue => println!("Color: Blue")
        }
        println!("Weight: {}", self.weight);
        println!("Dimentions: {}*{}*{}", self.dimentions.height, self.dimentions.width, self.dimentions.length);
    }
}


fn main() {

    let color = Color::Green;
    let dimentions: Dimentions = Dimentions::new(10, 12, 13);
    let weight: u32 = 120;

    let shipping_box: ShippingBox = ShippingBox::new(color, dimentions, weight);
    shipping_box.print();
}