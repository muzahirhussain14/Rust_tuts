/*
Struct Update Syntax:
Rust provides feature to initialize struct without having to write to much boilerplace code.

*/

#[derive(Debug)]
struct Particle {
    color: char,
    size: (u32, u32),
    position: (i32, i32),
    velocity: i32,
    direction: f32
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            color: 'r',
            size: (100, 100),
            position: (0,0),
            velocity: 0,
            direction: 0.0,
        }
    }
}

fn main() {

    let particle = Particle {           // assign a new value to the 'velocity', and get the default for all the other values.
        velocity: 5,
        ..Particle::default()
    };

    // similarly, we can use the object above to initialize another object
    let fast_particle = Particle {
        velocity: 100,
        ..particle
    };

    println!("{:?}", particle);
    println!("{:?}", fast_particle);
}