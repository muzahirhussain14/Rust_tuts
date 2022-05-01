// Passing closures to the function

fn function_that_takes_closure(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {

    let result = op(a,b);
    result
}

fn main() {

    let add_closure = Box::new(|a: i32, b: i32| -> i32 {
        println!("Executing Sum Closure");
        a + b
    });

    let name: String = "Muzahir".to_owned();

    // The 'move' keyword moves the variable into the Closure. In this case, it moves 'name' variable
    let add_closure = Box::new( move |a: i32, b: i32| -> i32 {

        println!("Executing Sum Closure -- Called by {}", name);
        a + b
    });

    function_that_takes_closure(55, 23, add_closure);
}
