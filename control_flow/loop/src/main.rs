fn main() {
    println!("Normal loop");
    loop_demo();

    println!("While loop");
    while_loop();
}


// normal loop

fn loop_demo() {
    let mut a = 0;

    loop {
        if a == 8 {
            break;
        }
        println!("{}", a);
        a = a + 1;
    }

}

// while loop


fn while_loop() {
    let mut a = 0;

    while a!= 5 {
        println!("{}", a);
        a = a + 1;
    }

}