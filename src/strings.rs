use std::string;

pub fn run() {
    let mut hello =String::from("hello ");


    // get length
    println!("print our length: {}", hello.len());

    // push on a char
    hello.push('W');

    // push on a str
    hello.push_str("orld");

    // cap in bytes

    println!("cap hello cap: {}", hello.capacity());
    println!("hello empty?: {}", hello.is_empty());
    // contains
    println!("hello contains world: {}", hello.to_lowercase().contains("world"));
    // replace 
    println!("replace: {}", hello.replace("World", "there"));

    //loop through string wiht whitespace 
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //create a string with a certain cap 

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}",s);

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    //println!("print hello: {}", hello);

}