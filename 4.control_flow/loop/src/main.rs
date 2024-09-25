fn main() {
    println!("Normal loop with continue and break");
    loop_demo();

    println!("While loop with continue and break");
    while_loop();

    println!("For loop with continue and break");
    for_loop();
}

// Normal loop demonstrating continue and break
fn loop_demo() {
    let mut a = 0;

    loop {
        if a == 10 {
            break; // Exit the loop if `a` is 10
        }
        if a % 2 == 0 {
            a += 1;
            continue; // Skip printing even numbers
        }
        println!("{}", a);
        a += 1;
    }
}

// While loop demonstrating continue and break
fn while_loop() {
    let mut a = 0;

    while a < 10 {
        if a % 3 == 0 {
            a += 1;
            continue; // Skip printing numbers divisible by 3
        }
        println!("{}", a);
        a += 1;
    }
}

// For loop demonstrating continue and break
fn for_loop() {
    for a in 0..10 {
        if a % 2 == 0 {
            continue; // Skip printing even numbers
        }
        if a == 7 {
            break; // Exit the loop when `a` is 7
        }
        println!("{}", a);
    }
}
