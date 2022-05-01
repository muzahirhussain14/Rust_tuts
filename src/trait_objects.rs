// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Cost {
    fn get_cost(&self) -> u64;
}

struct Carpet {
    price: u64
}
impl Cost for Carpet {
    fn get_cost(&self) -> u64 {
        self.price
    }
}

struct Tile {
    price: u64
}
impl Cost for Tile {
    fn get_cost(&self) -> u64 {
        self.price
    }
}

struct Wood {
    price: u64
}
impl Cost for Wood {
    fn get_cost(&self) -> u64 {
        self.price
    }
}

fn calculate_cost(materials: Vec<Box<dyn Cost>>) -> u64 {

    materials.iter().map(|material| material.get_cost()).sum()
}

fn main() {

    let carpet = Carpet {price: 50};
    let tile = Tile {price: 13};
    let wood = Wood {price: 7};

    let materials: Vec<Box<dyn Cost>> = vec![Box::new(carpet), Box::new(tile), Box::new(wood)];
    println!("Total Cost of Material: £{:?}", calculate_cost(materials));
}