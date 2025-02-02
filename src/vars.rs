pub fn run() {
    let name = "brad";
    let mut age: i32 = 37;
    age = 34;

    println!("my name is: {}. and i am {}", name, age);

    // define constants:
    const ID: i32 = 001;
    println!("id: {}", ID);

    // assgint multi vars
    let (my_name, my_age) = ("chris", 23);
    println!("my new name is {}, and i am {}",my_name, my_age);
}