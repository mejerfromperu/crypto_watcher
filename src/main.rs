//mod print;
//mod vars;
//mod types;
//mod strings;
//mod tuples;
//mod arrays;
mod vectors;
mod api_crypto;
//use std::result;
fn main() {

    vectors::run();
    

    //place holder fo debug
    // println!("{:?}", (12, true, "hello"))
    api_crypto::run();
}
