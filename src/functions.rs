pub fn run() {
    greeting("hello", "jane");

    //Bind function values to variables
    let get_sum = add(5, 5);
    println!("{}", get_sum);

    //closures
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure sum {}", add_nums(3, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
