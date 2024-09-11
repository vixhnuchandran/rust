fn main() {
    // Integer types (signed and unsigned)
    let signed_8_bit: i8 = -128; // Range: -128 to 127
    let unsigned_8_bit: u8 = 255; // Range: 0 to 255

    let signed_32_bit: i32 = 123456; // Default integer type in Rust
    let unsigned_32_bit: u32 = 123456;

    let signed_64_bit: i64 = -987654321; // 64-bit integer
    let unsigned_64_bit: u64 = 987654321;

    // Floating-point types (f32 and f64)
    let float_32_bit: f32 = 3.14; // 32-bit floating point
    let float_64_bit: f64 = 2.718; // Default floating point type

    // Boolean type
    let boolean_true: bool = true;
    let boolean_false: bool = false;

    // Character type
    let character: char = 'R'; // Single Unicode scalar value

    // String slice (&str) and String (owned, growable)
    let string_slice: &str = "Hello, world!"; // Immutable string slice
    let mut owned_string: String = String::from("Hello, Rust!"); // Mutable String

    // Modify the owned String
    owned_string.push_str(" It's awesome!");

    // Arrays: fixed-size list of elements (must be the same type)
    let integer_array: [i32; 3] = [1, 2, 3]; // Array of length 3
    let float_array: [f64; 4] = [1.1, 2.2, 3.3, 4.4];

    // Tuples: fixed-size collection of values with different types
    let tuple: (i32, f64, char) = (42, 6.28, 'Z');

    // Access tuple elements
    let (x, y, z) = tuple; // Destructuring
    let first_element = tuple.0; // Access by index

    // Slices: references to a part of an array
    let slice_of_array: &[i32] = &integer_array[1..]; // Slice from second element onward


    // Printing out all the values
    println!("Signed 8-bit: {}", signed_8_bit);
    println!("Unsigned 8-bit: {}", unsigned_8_bit);
    println!("Signed 32-bit: {}", signed_32_bit);
    println!("Unsigned 32-bit: {}", unsigned_32_bit);
    println!("Signed 64-bit: {}", signed_64_bit);
    println!("Unsigned 64-bit: {}", unsigned_64_bit);

    println!("Float 32-bit: {}", float_32_bit);
    println!("Float 64-bit: {}", float_64_bit);

    println!("Boolean true: {}", boolean_true);
    println!("Boolean false: {}", boolean_false);

    println!("Character: {}", character);

    println!("String slice: {}", string_slice);
    println!("Owned String: {}", owned_string);

    println!("Integer array: {:?}", integer_array);
    println!("Float array: {:?}", float_array);

    println!("Tuple: {:?}", tuple);
    println!("First element of tuple: {}", first_element);
    println!("Tuple destructured: {}, {}, {}", x, y, z);

    println!("Slice of array: {:?}", slice_of_array);



}
