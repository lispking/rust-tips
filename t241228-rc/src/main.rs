use std::rc::Rc;

struct Data {
    value: i32,
}

fn main() {
    let data1 = Rc::new(Data { value: 42 });
    let data2 = data1.clone();

    println!("Data1 value: {}", data1.value);
    println!("Data2 value: {}", data2.value);
}
