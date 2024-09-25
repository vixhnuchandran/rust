// Define the crate
fn main() {
    // We call functions from different modules within the main binary crate
    println!("Calling basic math operations...");
    
    let result = basic_operations::add(5, 3);
    println!("5 + 3 = {}", result);
    
    let result = basic_operations::sub(5, 3);
    println!("5 - 3 = {}", result);

    println!("\nCalling advanced math operations...");
    
    let result = advanced_operations::multiply(5, 3);
    println!("5 * 3 = {}", result);

    let result = advanced_operations::divide(6, 2);
    match result {
        Ok(value) => println!("6 / 2 = {}", value),
        Err(err) => println!("Error: {}", err),
    }
}

// Define a module for basic math operations
mod basic_operations {
    // Public function in the module
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

// Define another module for advanced math operations
mod advanced_operations {
    // A nested module inside advanced_operations
    pub mod error_handling {
        pub fn divide_by_zero() -> &'static str {
            "Cannot divide by zero!"
        }
    }

    // Public function in the module
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            Err(error_handling::divide_by_zero())
        } else {
            Ok(a / b)
        }
    }
}
