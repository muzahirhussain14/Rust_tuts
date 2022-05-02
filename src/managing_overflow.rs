/*
Managing Overflowing
    - Checked_*()
    - Overflowing_*()
    - Saturating_*()
    - Wrapping_*()
*/

fn main(){

    // Checked_* -> Returns Some with calculation result or None if overflow occured
    let n1: Option<u32> = 0u32.checked_sub(1);                           // results in overflow, thus none will be returned
    let n2: Option<u32> = u32::MAX.checked_add(1);                       // results in overflow
    let n3: Option<u32> = 9_u32.checked_add(1);                          // ok
    println!("Checked_* Results: {:?}, {:?}, {:?}", n1, n2, n3);

    // Overflowing_* -> Returns a tuple of calculation result and a boolean. Boolean is true if overflow occured otherwise it returns false.
    let n1: (u32, bool) = 0u32.overflowing_sub(1);
    let n2: (u32, bool) = u32::MAX.overflowing_add(1);
    let n3: (u32, bool) = 9_u32.overflowing_add(1);
    println!("Overflowing_* Results: {:?}, {:?}, {:?}", n1, n2, n3);    

    // Saturating_* -> ensure that the values stays between the min and max bound of the datatype.
    let n1: u32 = 0_u32.saturating_sub(9001);                           // overflow occured and so result will always stay at 0
    let n2: u32 = u32::MAX.saturating_add(u32::MAX);                    // overflow occured and so the result will stay at the max
    println!("saturating_* Results: {:?}, {:?}", n1, n2);    

    // Wrapping_* -> it wraps the value around. It is the default behaviour in the release mode.
    let n1: u32 = 1_u32.wrapping_sub(2);                                 // will be wrapped to the max value
    let n2: u32 = u32::MAX.wrapping_add(1);                              // will be wrapped to 0
    println!("Wrapping_* Results: {:?}, {:?}", n1, n2);    
}