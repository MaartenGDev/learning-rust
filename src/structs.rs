struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct TupleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}


impl Person {
    fn new(first: &str, last: &str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 12,
        blue: 33,
    };

    c.green = 200;


    println!("r: {}, g: {} b: {}", c.red, c.green, c.blue);

    let mut ts = TupleColor(255, 12, 33);

    println!("r: {}, g: {} b: {}", ts.0, ts.1, ts.2);

    let mut p = Person::new("John", "Joe");
    p.set_last_name("Williams");

    println!("Person: {}, {}", p.first_name, p.last_name);
    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.to_tuple())

}