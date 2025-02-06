pub fn run() {

    // greeting("chris", "mejer");


    let get_sum = add(5, 4);
    println!("sum : {}", get_sum)
}

fn greeting (greet: &str, name: &str) {
    println!("{} - {}, nice to meet you", greet, name)
}

fn add (n1: i32, m2: i32) -> i32 {
    n1 + m2
}