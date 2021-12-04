pub fn run() {
    //Default is "i32"
    let x = 1;

    //Default is f64
    let y = 2.5;

    //Adding explicit type
    let z: i64 = 4221421421;

    //Finding max size of datatype
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    //Boolean
    let is_active = 10 < 5;

    let a = "😎";

    println!("{:?}", (x, y, z, is_active, a));
}
