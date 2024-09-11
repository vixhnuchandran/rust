fn main() {
    // 1. Ownership Basics
    let s1 = String::from("Hello"); // `s1` owns the String
    let s2 = s1; // Ownership of the String is transferred to `s2`
    // println!("{}", s1); // This line would cause a compile-time error because `s1` no longer owns the String

    // 2. Scope and Cleanup
    {
        let s3 = String::from("World"); // `s3` owns the String
        println!("{}", s3); // This works because `s3` owns the String
    } // `s3` goes out of scope here and the memory is freed

    // `s3` is no longer accessible beyond this point

    // 3. Function Ownership
    let s4 = String::from("Function Ownership");
    takes_ownership(s4); // Ownership of `s4` is transferred to the function
    // println!("{}", s4); // This would cause a compile-time error because `s4` no longer owns the String
}

fn takes_ownership(s: String) {
    println!("Inside function: {}", s); // `s` is used here within the function
    // Ownership of `s` is dropped when the function ends
}
