use std::collections::HashMap;

fn main() {
    // 1. Creating a new empty HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // 2. Inserting key-value pairs into the HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 3. Accessing values using the `get` method (returns Option<&V>)
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("Score for {}: {}", team_name, score), // Output: Score for Blue: 10
        None => println!("Team not found"),
    }

    // 4. Updating a value by replacing it
    scores.insert(String::from("Blue"), 25);
    println!("Updated scores: {:?}", scores); // Output: {"Blue": 25, "Yellow": 50}

    // 5. Checking if a key exists before inserting (only inserts if key doesn't exist)
    scores.entry(String::from("Blue")).or_insert(30); // Won't insert as "Blue" exists
    scores.entry(String::from("Green")).or_insert(30); // Inserts "Green"
    println!("After using entry: {:?}", scores); // Output: {"Blue": 25, "Yellow": 50, "Green": 30}

    // 6. Iterating over the HashMap
    for (key, value) in &scores {
        println!("Team: {}, Score: {}", key, value); // Output: Each team and its score
    }

    // 7. Updating values based on the old value (modifying the value)
    for (_key, value) in scores.iter_mut() {
        *value += 10; // Adding 10 to each score
    }
    println!("Scores after adding 10: {:?}", scores); // Output: {"Blue": 35, "Yellow": 60, "Green": 40}

    // 8. Removing a key-value pair
    scores.remove(&String::from("Green"));
    println!("Scores after removing Green: {:?}", scores); // Output: {"Blue": 35, "Yellow": 60}

    // 9. Checking if a HashMap contains a key
    if scores.contains_key("Yellow") {
        println!("Yellow team exists!"); // Output: Yellow team exists!
    }

    // 10. Using HashMap with types other than String and i32
    let mut city_population: HashMap<&str, u64> = HashMap::new();
    city_population.insert("Tokyo", 37_400_068);
    city_population.insert("Delhi", 28_514_000);
    city_population.insert("Shanghai", 25_582_000);

    println!("City populations: {:?}", city_population); // Output: city names and their populations

    // 11. Merging two HashMaps using extend (adds key-value pairs from another HashMap)
    let mut more_scores = HashMap::new();
    more_scores.insert(String::from("Red"), 45);
    more_scores.insert(String::from("Blue"), 40); // This will overwrite the existing "Blue" team score

    scores.extend(more_scores); // Merge more_scores into scores
    println!("Scores after extend: {:?}", scores); // Output: {"Blue": 40, "Yellow": 60, "Red": 45}

    // 12. Clearing the HashMap (removes all key-value pairs)
    scores.clear();
    println!("Scores after clear: {:?}", scores); // Output: {}
}
