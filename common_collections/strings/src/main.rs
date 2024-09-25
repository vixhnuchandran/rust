fn main() {
    // 1. Creating a new empty String
    let mut s = String::new();

    // 2. Appending a string slice using `push_str`
    s.push_str("Hello");
    println!("After push_str: {}", s); // Output: Hello

    // 3. Appending a single character using `push`
    s.push(' ');
    s.push('R');
    s.push('u');
    s.push('s');
    s.push('t');
    println!("After push: {}", s); // Output: Hello Rust

    // 4. Concatenating strings using the `+` operator (moves ownership)
    let s2 = String::from(" is awesome!");
    let s3 = s + &s2; // s is moved and can no longer be used
    println!("Concatenated string: {}", s3); // Output: Hello Rust is awesome!

    // 5. Using `format!` macro for concatenation without moving ownership
    let s4 = String::from("Hello");
    let s5 = String::from(" Rust");
    let s6 = String::from(" is cool!");
    let combined = format!("{}{}{}", s4, s5, s6); // s4, s5, s6 are not moved
    println!("Formatted string: {}", combined); // Output: Hello Rust is cool!

    // 6. Accessing individual characters (Note: Rust strings are UTF-8, so indexing is not allowed)
    for c in s4.chars() {
        println!("Character: {}", c); // Output: H e l l o
    }

    // 7. Slicing a string by byte index (careful with UTF-8 boundaries)
    let hello = "Здравствуйте"; // UTF-8 string
    let slice = &hello[0..4]; // Slicing first two bytes (one character in Cyrillic)
    println!("Sliced string: {}", slice); // Output: Зд (2 characters)

    // 8. Getting the length of a string in bytes
    println!("Length of '{}' in bytes: {}", hello, hello.len()); // Output: Length of 'Здравствуйте' in bytes: 24

    // 9. Iterating over a string by characters
    for c in hello.chars() {
        println!("Character: {}", c); // Output: З д р а в с т в у й т е
    }

    // 10. Iterating over a string by bytes
    for b in hello.bytes() {
        println!("Byte: {}", b); // Output: UTF-8 encoded bytes
    }

    // 11. Converting a string slice to a String using `to_string` and `from` methods
    let slice = "This is a slice";
    let string_from_slice = slice.to_string();
    let string_from_slice2 = String::from(slice);
    println!("Converted string: {}", string_from_slice); // Output: This is a slice
    println!("Converted string 2: {}", string_from_slice2); // Output: This is a slice

    // 12. Clearing the string (removes all content, but keeps allocated memory)
    let mut temp_string = String::from("Temporary String");
    temp_string.clear();
    println!("Cleared string: '{}'", temp_string); // Output: ''

    // 13. Checking if a string is empty
    println!("Is the string empty? {}", temp_string.is_empty()); // Output: true

    // 14. Replacing part of the string using `replace`
    let original = String::from("I love Rust!");
    let replaced = original.replace("love", "enjoy");
    println!("Replaced string: {}", replaced); // Output: I enjoy Rust!

    // 15. Checking if a string contains a substring
    let contains = replaced.contains("Rust");
    println!("Does the string contain 'Rust'? {}", contains); // Output: true

    // 16. Splitting a string by whitespace and collecting into a vector
    let sentence = String::from("Hello world, Rust is great!");
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("Words: {:?}", words); // Output: ["Hello", "world,", "Rust", "is", "great!"]

    // 17. Trimming whitespace from both ends of a string
    let padded = String::from("   Lots of space   ");
    let trimmed = padded.trim();
    println!("Trimmed string: '{}'", trimmed); // Output: 'Lots of space'
}
