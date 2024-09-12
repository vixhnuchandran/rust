fn main() {
    // Slicing an array
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // slice of `[2, 3]`
    println!("Original array: {:?}", arr);
    println!("Array slice: {:?}", slice);

    // Slicing a string
    let s = String::from("Hello, Rust!");
    let slice_str = &s[0..5]; // slice of `"Hello"`
    println!("Original string: {}", s);
    println!("String slice: {}", slice_str);

    // Slicing a string slice (&str)
    let s2 = "Rustacean";
    let slice_str2 = &s2[0..4]; // slice of `"Rust"`
    println!("Original &str: {}", s2);
    println!("&str slice: {}", slice_str2);
}
