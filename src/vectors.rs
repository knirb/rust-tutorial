pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Reassigning a value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Popping last value of vector
    numbers.pop();

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

    //Looping through vector values
    for x in numbers.iter() {
        println!("{}", x);
    }

    //Loop & Mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
