/*
    Options combinator practice
*/

fn main() {
    let list: Vec<_> = vec![3,2,1,4];

    // 1) multiply each element by 3 -- maps
    let x: Vec<_> = list.iter().map(|f| f * 3).collect();
    println!("Multiply each element by 3: {:?}", x);

    // 2) get elements greater or equals to 9 -- maps and filters
    let x: Vec<_> = list.iter().map(|f| f * 3).filter(|f| f >= &9 ).collect();
    println!("Elements greater than or equal to 9: {:?}", x);

    // 3) count element
    let x = list.iter().count();
    println!("Total elements: {:?}", x);

    // 4) find element
    let x= list.iter().find(|n| n == &&9);
    println!("Find element: {:?}", x);

    // 5) Min element
    let x = list.iter().min();
    println!("Minimum element: {:?}", x);

    // 6) Maximum element
    let x = list.iter().max();
    println!("Maximum element: {:?}", x);

}