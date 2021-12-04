use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Viktor";
    println!("Args: {}", command);

    if command == "hello" {
        println!("Hello {}", name)
    } else if command == "status" {
        println!("順調に進んでいます")
    }
}
