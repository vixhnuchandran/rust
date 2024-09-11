#[allow(dead_code)]  // Suppresses the warning about unused variants

enum Direction {
    North,
    South,
    West,
    East
}

fn main() {

    let chosen_direction = Direction::West;
    which_direction(chosen_direction);


}

fn which_direction(dir: Direction) {
    match dir {
        Direction::North => println!("go up"),
        Direction::South => println!("go down"),
        Direction::West => println!("go left"),
        Direction::East => println!("go right"),

    }
}