/*
Topic: Testing
Requirements:
    Write tests for the existing program to ensure proper functionality.

Notes:
    - Create atleast two test cases for each function.
    - Use 'cargo test' to test the program.
    - There are intentional bugs in the program that needs to be fixed
        - Check the documentation commands of the functions to check how they should operate
*/


// Ensures n >= lower and <= upper
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

#[cfg(test)]
mod test {

    use crate::*;

    // testing clamp method
    #[test]
    fn clamp_test() {
        let n = 25;
        let lower = 15;
        let upper = 30;

        let result = clamp(25, 15, 30);
        let expected = n;
        assert_eq!(result, expected);
    }

    // test concat
    #[test]
    fn concat_test() {
        let a = "abc";
        let b = "def";
        let result = concat(a, b);
        let expected = "abcdef";
        assert_eq!(result, expected);
    }
}

fn main() {

}