
pub fn run() {
    
    let age:u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink ?")
    } else if age < 21 && check_id {
        println!("Sorry you have to leave.. ")
    } else {
        println!("Bartedner: ill need to check ur id")
    } 


    //shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age: {}",is_of_age)
}