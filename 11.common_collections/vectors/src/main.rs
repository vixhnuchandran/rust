fn main() {
    // 1. Creating a new, empty vector that will hold i32 integers
    let mut vec: Vec<i32> = Vec::new();

    // 2. Adding elements to the vector using the `push` method
    vec.push(10);
    vec.push(20);
    vec.push(30);

    // 3. Accessing elements by index (panics if index is out of bounds)
    println!("First element: {}", vec[0]); // Output: 10

    // 4. Safely accessing elements using `get` (returns Option<&T>)
    match vec.get(2) {
        Some(value) => println!("Third element: {}", value), // Output: 30
        None => println!("Index out of bounds"),
    }

    // 5. Removing the last element using `pop` (returns Option<T>)
    let last_element = vec.pop();
    println!("Popped element: {:?}", last_element); // Output: Some(30)

    // 6. Inserting an element at a specific index (shifts other elements to the right)
    vec.insert(1, 15);
    println!("Vector after insert: {:?}", vec); // Output: [10, 15, 20]

    // 7. Removing an element at a specific index using `remove`
    let removed_element = vec.remove(1);
    println!("Removed element: {}", removed_element); // Output: 15
    println!("Vector after removal: {:?}", vec); // Output: [10, 20]

    // 8. Iterating over the vector by borrowing elements
    for val in &vec {
        println!("Value: {}", val); // Output: 10, 20
    }

    // 9. Iterating mutably, modifying elements in place
    for val in &mut vec {
        *val += 5; // Adding 5 to each element
    }
    println!("Modified vector: {:?}", vec); // Output: [15, 25]

    // 10. Cloning a vector (deep copy of the elements)
    let cloned_vec = vec.clone();
    println!("Cloned vector: {:?}", cloned_vec); // Output: [15, 25]

    // 11. Getting the length of the vector using `len`
    println!("Length of vector: {}", vec.len()); // Output: 2

    // 12. Using `capacity` to see how much space the vector has allocated
    println!("Capacity of vector: {}", vec.capacity());

    // 13. Shrinking the capacity to match the length using `shrink_to_fit`
    vec.shrink_to_fit();
    println!("Capacity after shrink: {}", vec.capacity());

    // 14. Clearing the vector to remove all elements
    vec.clear();
    println!("Vector after clear: {:?}", vec); // Output: []
    println!("Is the vector empty? {}", vec.is_empty()); // Output: true
}
