
pub fn run() {
    println!("max i32 {}", std::i32::MAX);


    let is_active = true;
    println!("{:?}", (is_active));

    // get bool from experession
    let is_greater: bool = 10> 5;
    println!("{:?}", (is_greater));

    // char

    let a1:char = 'a';
    let face:char = '\u{1F600}';
    println!("first a char: {}, and then a another char: {}", a1, face)
}