pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // Reassigning a value
    numbers[2] = 20;

    // Getting a single value
    println!("Value {}", numbers[0]);

    println!("{:?}", numbers);
    // Getting length
    println!("{}", numbers.len());

    // Getting memory allocated by array
    println!("{}", std::mem::size_of_val(&numbers));

    // Getting slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);
}
