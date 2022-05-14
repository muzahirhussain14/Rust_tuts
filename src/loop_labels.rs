/*
Loop labels:

Loop labels allows us to name the loops and break/continue from them.
it works for all the three loops, loop, for, while

    'ident: loop {}
    'ident: for x in y {}
    'ident: while true {}
*/

fn main() {

    let matrix = [
        [1,4,6],
        [4,2,20],
        [12,2,7]
    ];

    'rows: for row in matrix.iter() {
        'cols: for col in row {
            if col == &2 {
                continue 'rows;                // continue to the 'rows' loop
            }
            println!("{}", col);
        }
    }
}