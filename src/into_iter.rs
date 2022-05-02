/*
Into_Iter() => Value is moved into the iterator
We can also borrow the values using Into_Iter().
Following are all the three implementation of the IntoIterator, i.e. moved, borrowed, mutable borrow.
*/

struct Friends {
    names: Vec<String>,
}


// Values are moved into the iterator using this implementation
impl IntoIterator for Friends {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

// values are borrowed using this implementation
impl<'a> IntoIterator for &'a Friends {

    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.iter()
    }
}

// values are mutabily borrowed using this implementation
impl<'a> IntoIterator for &'a mut Friends {

    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}

fn main() {

    let mut friends = Friends {names: vec!["Kazim".to_owned(), "Muzahir".to_owned(), "Ali".to_owned()]};

    // values are borrowed in the iterator
    println!("\nBorrowed Values");
    for f in &friends {
        println!("{}", f);
    }

    // values are mutabily borrowed into the iterator
    println!("\nBorrowed Values mutabily");
    for f in &mut friends {
        println!("{}", f);
    }

    // into_iter that we have implemented is called here and the values are moved.
    println!("\nMoved Values");
    for f in friends {
        println!("{}", f);
    }
}