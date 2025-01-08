struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

    fn get_item(&self) -> &T {
        &self.item
    }
}

fn main() {
    let int_container = Container::new(42);
    let string_container = Container::new("Hello");
    println!(
        "int: {:?}, string: {:?}",
        int_container.get_item(),
        string_container.get_item()
    );
}
