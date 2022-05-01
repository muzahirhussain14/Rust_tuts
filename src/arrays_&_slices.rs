// Topic: Arrays & Slices
//
// Requirements:
// * Print pairs of numbers and their sums as they are streamed from a data source
// * If only one number is received, then print "Unpaired value: V",
//   where V is the value
// * If no numbers are received, print "Data stream complete"
//
// Notes:
// * A simulated data stream is already configured in the code
// * See the stdlib docs for the "chunks" method on "slice" for more info

type xyz = String;

fn data() -> &'static [u64]
{
    &[5,5,4,4,3,3,2,2,1]
}

fn sum_up(chunk: &[u64]) {

    match chunk {
        [lhs, rhs] => println!("{} + {} = {}", lhs, rhs, (lhs + rhs)),
        [first] => println!("Unpaired Value: {}", first),
        [] => println!("Data stream complete"),
        [..] => unreachable!("Exaustive data")
    }
}

fn print_array(array: &mut [u8; 8]) {

    println!("array is: {:?}", array);
    array[0] = 5;
}

fn main() {
    
    let data = data().chunks(2);
    
    for chunk in data {
        sum_up(chunk)
    }

    let str: xyz = "Hello World".to_owned();
    println!("Value: {}", str);

    let mut arr: [u8; 8] = [2,3,54,6,34,2,1,0];
    print_array(&mut arr);
    println!("array is: {:?}", arr);
}