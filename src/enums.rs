enum Movement {
    up,
    down,
    left,
    right,
}

fn move_character(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::up => println!("Moving up"),
        Movement::down => println!("Moving down"),
        Movement::left => println!("Moving left"),
        Movement::right => println!("Moving right"),
    }
}

pub fn run() {
    let avatar1 = Movement::left;
    let avatar2 = Movement::right;
    let avatar3 = Movement::up;
    let avatar4 = Movement::down;

    move_character(avatar1);
    move_character(avatar2);
    move_character(avatar3);
    move_character(avatar4);
}
