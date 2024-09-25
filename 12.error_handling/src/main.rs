// Function to perform division with Result for error handling
fn divide(dividend: i32, divisor: i32) -> Result<i32, &'static str> {
    if divisor == 0 {
        Err("Cannot divide by zero") // Return an error if divisor is zero
    } else {
        Ok(dividend / divisor) // Return the result wrapped in Ok
    }
}

// Function to perform division with Option for error handling
fn divide_with_option(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None // Return None if divisor is zero
    } else {
        Some(dividend / divisor) // Return the result wrapped in Some
    }
}

fn main() {
    // Using Result
    let result1 = divide(10, 2);
    let result2 = divide(10, 0);

    // Handling Result with match
    match result1 {
        Ok(quotient) => println!("Result 1: {}", quotient),
        Err(e) => println!("Error: {}", e),
    }

    match result2 {
        Ok(quotient) => println!("Result 2: {}", quotient),
        Err(e) => println!("Error: {}", e),
    }

    // Using Option
    let result3 = divide_with_option(10, 2);
    let result4 = divide_with_option(10, 0);

    // Handling Option with match
    match result3 {
        Some(quotient) => println!("Result 3: {}", quotient),
        None => println!("Error: Cannot divide by zero"),
    }

    match result4 {
        Some(quotient) => println!("Result 4: {}", quotient),
        None => println!("Error: Cannot divide by zero"),
    }
}
