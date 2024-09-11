fn main() {
    
    // boolean match 
    let demo_bool = false;
    match demo_bool {
        true => println!("It's true"),
        false => println!("It's false"),

    }

    // int match 
    let demo_int = 4;
    match demo_int {
        1 => println!("It's 1"), 
        2 => println!("It's 2"), 
        3 => println!("It's 3"), 
        4 => println!("It's 4"), 
        _ => println!("It's greater than 4"), 

    }
}


