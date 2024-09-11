fn main() {
    // Example of function calls and their results
    let add_result = add(5, 3);
    println!("Addition result is : {}", add_result);

    let sub_result = sub(20, 17);
    println!("Subtraction result is : {}", sub_result);

    let mul_result = mul(5, 2);
    println!("Multiplication result is : {}", mul_result);

    let div_result = div(10, 2);
    println!("Division result is : {}", div_result);

    // Example of a function with a unit return type
    print_message(); // This function doesn't return a value, it has a unit return type
}

// 1. Defining functions (fn keyword)
// Functions are defined using the `fn` keyword. The function signature includes the function name, parameters, and return type.

// Function to add two integers
fn add(num1: i32, num2: i32) -> i32 {
    // The function returns the sum of num1 and num2
    num1 + num2 // This is an expression that calculates the sum
}

// Function to subtract one integer from another
fn sub(num1: i32, num2: i32) -> i32 {
    // The function returns the difference between num1 and num2
    num1 - num2 // This is an expression that calculates the difference
}

// Function to multiply two integers
fn mul(num1: i32, num2: i32) -> i32 {
    // The function returns the product of num1 and num2
    num1 * num2 // This is an expression that calculates the product
}

// Function to divide one integer by another
fn div(num1: i32, num2: i32) -> i32 {
    // The function returns the quotient of num1 divided by num2
    num1 / num2 // This is an expression that calculates the quotient
}

// 3. Expressions vs. Statements in Functions
// In Rust, an expression evaluates to a value, while a statement performs some action but does not produce a value.

// Function with a unit return type (no value returned)
fn print_message() {
    println!("This function returns nothing."); // `println!` is a statement
    // The function has no return value, its return type is `()`, the unit type.
}
