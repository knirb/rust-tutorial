pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    //Printing variables/combining strings
    println!("Number: {}", 1);

    //Multiple
    println!("{} is a {}", 1, 2);

    //Positional Arugments
    println!(
        "{0} is from {1} and {0} is a real {2}",
        "Brad", "Sandviken", "douche"
    );

    //Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Viktor",
        activity = "dota"
    );

    //Placeholder traits
    println!("Binary: {:b},  hex:{:x}, Octal: {:o}", 10, 10, 10);

    //Placeholder from debug trait
    println!("{:?}", (12, true, "hello"));
}
