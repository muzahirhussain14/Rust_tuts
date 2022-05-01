/*
RefCell
- Difference between cells and refcells is that refcell data always results in a borrow, while a cell resulted in move/copy
- More efficient then cell
- Both Cell and Refcell is not threadsafe. Can only be used in a single thread.
*/

use std::cell::{Cell, RefCell};

struct Person {
    name: RefCell<String>
}

fn main() {

    let name = "Muzahir".to_owned();
    let person = Person{name: RefCell::new(name)};

    let mut name = person.name.borrow_mut();
    *name = "Hussain".to_owned();                                   // modifying the person's data, despite the fact that it is not declared mutable
}