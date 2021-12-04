pub fn run() {
    //normal string
    let hello = "Hello";

    //Growable string, mutables
    let mut world = String::from("World!");

    //String works like javascript Primitives in that we can use methods like:
    println!("{}", world.len());

    world.push('!');
    world.push_str("!");

    //Getting Capacity in bytes
    println!("Capacity: {}", world.capacity());

    println!("{}, {}", hello, world);

    //other useful methods
    println!("isEmpty, {}", world.is_empty());

    for word in world.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);
}
