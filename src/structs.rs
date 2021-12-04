struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// Tuple struct
struct Clr(u8, u8, u8);

struct Person {
    f_name: String,
    l_name: String,
}

// Struct implementation
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            f_name: first.to_string(),
            l_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.f_name, self.l_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.l_name = last.to_string();
    }
}

pub fn run() {
    let mut c = Color { r: 255, g: 0, b: 0 };

    println!("color: {} {} {}", c.r, c.g, c.b);

    c.r = 100;

    println!("color: {} {} {}", c.r, c.g, c.b);

    let mut clr = Clr(255, 0, 0);
    println!("color: {} {} {}", clr.0, clr.1, clr.2);
    clr.0 = 100;
    println!("color: {} {} {}", clr.0, clr.1, clr.2);
    let mut me = Person::new("Viktor", "Åsbrink");

    println!("My name is {}", me.full_name());

    me.set_last_name("Söderström");

    println!("My name is {}", me.full_name());
}
