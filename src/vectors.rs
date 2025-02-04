use std::mem;
pub fn run() {
    let mut numbers: [i32;4] = [1,2,3,4];

    println!("{:?}", numbers);
    
    // get single val
    println!("Single val: {:?}", numbers[0]);

    // reassing
    numbers[2] = 20;
    println!("{:?}", numbers);

    // get legnth
    println!("{:?}", numbers.len());


    // arrays are stack allocated 
    println!("array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);
}