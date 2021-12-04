// Tuples group together values of different types

pub fn run() {
    let person: (&str, &str, i8) = ("Viktor", "Åsbrink", 29);

    println!("{} {} is {} old", person.0, person.1, person.2);
}
