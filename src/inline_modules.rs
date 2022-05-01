/*
Topic: Inline Modules
*/

mod msg {
    use std::fmt::format;

    pub fn trim(msg: &str) -> &str {
        msg.trim()
    }

    // fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
    //     if let Some(letter) = msg.get(0..1) {
    //         format!("{}{}", letter.to_uppercase(), &msg[1..msg.len()]).as_str()
    //     }
    //     else {
    //         msg.into()
    //     }
    // }

    pub fn exciting(msg: &str) ->  String {
        format!("{}!",  msg)
    }
}

mod math{

    pub fn add(lhs: isize, rhs: isize) -> isize {
        lhs + rhs
    }

    pub fn sub(lhs: isize, rhs: isize) -> isize {
        lhs - rhs
    }

    pub fn mul(lhs: isize, rhs: isize) -> isize {
        lhs * rhs
    }
}
fn main() {

    // Part 1: math functions
    let result = {
        let two_plus_two = math::add(2,2);
        let three = math::sub(two_plus_two, 1);
        math::mul(three, three);
    };

    // Ensure we have the correct result

    {
        use msg::{trim, exciting};
        //use msg::*;

        let hello = {
            let msg = "hello ";
            trim(msg)
        };
        let world = {
            let msg = "world";
            exciting(msg)
        };
        let msg = format!("{:?},{:?}", hello, world);
    
        // ensure we have a correct result
        assert_eq!(&msg, "Hello, World");
        print!("{}", msg);
    }
}