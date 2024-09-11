// Example of various struct concepts in Rust

// 1. Defining a basic struct
struct Person {
    name: String,
    age: u32,
}

// 2. Defining a tuple-like struct
struct Point(f64, f64);

// 3. Struct methods
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to create a new instance
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// 4. Structs with default values
#[derive(Default)]
struct Config {
    host: String,
    port: u16,
}

// 5. Structs with private fields
mod shapes {
    pub struct Circle {
        radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }

        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
}

// 6. Destructuring structs
struct DestructPerson {
    name: String,
    age: u32,
}

// 7. Tuple structs
struct Color(u8, u8, u8);

// 8. Unit-like structs
struct UnitStruct;

fn main() {
    // 1. Using a basic struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Person: Name: {}, Age: {}", person.name, person.age);

    // 2. Using a tuple-like struct
    let point = Point(3.0, 4.0);
    println!("Point: x: {}, y: {}", point.0, point.1);

    // 3. Using struct methods
    let rect = Rectangle::new(30, 50);
    println!("Rectangle area: {} square pixels", rect.area());

    // 4. Using default values
    let default_config = Config::default();
    println!("Default Config: Host: {}, Port: {}", default_config.host, default_config.port);

    // 5. Using private fields
    let circle = shapes::Circle::new(5.0);
    println!("Circle area: {}", circle.area());

    // 6. Destructuring structs
    let person = DestructPerson {
        name: String::from("Bob"),
        age: 25,
    };
    let DestructPerson { name, age } = person;
    println!("Destructured Person: Name: {}, Age: {}", name, age);

    // 7. Using tuple structs
    let color = Color(255, 0, 0);
    println!("Color: Red: {}, Green: {}, Blue: {}", color.0, color.1, color.2);

    // 8. Using unit-like structs
    let _unit = UnitStruct;
    println!("This is a unit-like struct.");
}
