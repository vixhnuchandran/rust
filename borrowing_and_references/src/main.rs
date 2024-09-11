fn main() {
    // Example of immutable references
    let s1 = String::from("Hello, Rust!");

    // Creating multiple immutable references to `s1`
    let r1 = &s1; // First immutable reference
    let r2 = &s1; // Second immutable reference

    // Both references can be used to read the value of `s1`
    println!("Immutable reference 1: {}", r1);
    println!("Immutable reference 2: {}", r2);

    // Example of mutable references
    let mut s2 = String::from("Mutable Reference");

    // Creating a mutable reference to `s2`
    let r3 = &mut s2;

    // Mutable reference allows modifying the value of `s2`
    r3.push_str(" - Modified");
    println!("Mutable reference: {}", r3);

    // `r3` goes out of scope here, allowing `s2` to be used again
    // Example of reusing the original variable after mutable reference
    println!("Original variable after modification: {}", s2);

    // Example of borrowing rules
    let mut s3 = String::from("Borrowing Rules");

    // Immutable references
    let r4 = &s3; // First immutable reference
    let r5 = &s3; // Second immutable reference

    // Uncommenting the following line will cause a compile-time error
    // because mutable references cannot coexist with immutable ones:
    // let r6 = &mut s3; // This line would cause a compile-time error

    // Mutable reference must be the only reference at this point
    let r6 = &mut s3; // Creating a mutable reference after immutable ones are done
    r6.push_str(" - Modified");

    // `r6` is the only reference to `s3` now, so we can use it
    println!("Modified value: {}", r6);
}
