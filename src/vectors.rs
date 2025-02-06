use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    println!("{:?}", numbers);
    
    // get single val
    println!("Single val: {:?}", numbers[0]);

    // reassing
    numbers[2] = 20;
    println!("{:?}", numbers);

    numbers.push(5);
    numbers.push(6);


    numbers.pop();
    // get legnth
    println!("{:?}", numbers.len());


    // vectors are stack allocated 
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);


    // loop throughtvectors values
    for x in numbers.iter(){
        println!("number : {}", x)
    }

    //loop and mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("numbers vex: {:?}", numbers)
}