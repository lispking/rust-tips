trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    age: u8,
}

impl Printable for Person {
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };

    person.print();
}
