#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl From<(String, u32)> for Person {
    fn from(tuple: (String, u32)) -> Self {
        Person {
            name: tuple.0,
            age: tuple.1,
        }
    }
}

impl From<Person> for (String, u32) {
    fn from(val: Person) -> Self {
        (val.name, val.age)
    }
}

fn main() {
    let tuple = ("Alice".to_string(), 25);
    let person = Person::from(tuple);
    println!("{:?}", person);

    let person = Person {
        name: "Bob".to_string(),
        age: 30,
    };
    let tuple: (String, u32) = person.into();
    println!("{:?}", tuple);
}
