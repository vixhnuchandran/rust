#[allow(dead_code)] // Suppresses the warning about unused variants
enum Direction {
    North,
    South,
    West,
    East,
    // Enum with data
    CustomDirection { x: i32, y: i32 },
    // Enum with multiple data types
    Angle(f64),
}

fn main() {
    let chosen_direction = Direction::West;
    let custom_direction = Direction::CustomDirection { x: 10, y: 20 };
    let angle_direction = Direction::Angle(45.0);

    which_direction(chosen_direction);
    which_direction(custom_direction);
    which_direction(angle_direction);

    // Match guards example
    let angle = Direction::Angle(60.0);
    match angle {
        Direction::Angle(a) if a < 45.0 => println!("Acute angle"),
        Direction::Angle(a) if a == 45.0 => println!("Right angle"),
        Direction::Angle(a) if a > 45.0 => println!("Obtuse angle"),
        _ => println!("Not an angle"),
    }
}

// Match on enum variants with data and pattern matching
fn which_direction(dir: Direction) {
    match dir {
        Direction::North => println!("go up"),
        Direction::South => println!("go down"),
        Direction::West => println!("go left"),
        Direction::East => println!("go right"),
        Direction::CustomDirection { x, y } => {
            println!("Custom direction with x: {}, y: {}", x, y);
        }
        Direction::Angle(angle) => {
            println!("Angle direction: {} degrees", angle);
        }
    }
}
