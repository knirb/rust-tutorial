pub fn run() {
    let name = "Brad";
    let age = 37;
    println!("Name: {}, age: {}", name, age);

    //mutable varaibles
    let mut age2 = 30;
    age2 = 40;
    println!("Mutable age: {}", age2);

    //Defining constants, type must be explicitly set
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assigning multiple variables at once
    let (a, b) = ("Viktor", 29);
    println!("{:?}", (a, b));
}
