/*
Cell and RefCell
    Allows you to mutate shared data.

Cell
    Permenently mutable memory location
    - Can always be mutated, even if the containing structure is immutable
    - Accessing cell data always results in a move or copy
    - Data should be copy-able 
        - #derive(Clone, Copy)
    - Inefficient for large data types (limit to booleans and numbers)
    - Prefer mut
*/

use std::cell::Cell;

#[derive(Debug)]
struct Book {
    signed: Cell<bool>
}
impl Book {
    fn sign(&self) {
        self.signed.set(true);
    }

    fn is_signed(&self) -> bool {
        self.signed.get()
    }
}
fn main() {

    let book = Book {signed: Cell::from(false)};

    /*
    We didn't declare book 'mut' yet still we are able to change its attribute. This is because of Cell which allows use to mutate the shared data.
    */
    println!("Book (before signing): {:?}", book);
    book.sign();
    println!("Book (before signing): {:?}", book);
}