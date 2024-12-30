use std::sync::Arc;
use std::thread;

struct Data {
    value: i32,
}

fn main() {
    let shared_data = Arc::new(Data { value: 42 });

    let shared_data1 = Arc::clone(&shared_data);
    let thread1 = thread::spawn(move || {
        println!("Thread 1: {}", shared_data1.value);
    });

    let shared_data2 = Arc::clone(&shared_data);
    let thread2 = thread::spawn(move || {
        println!("Thread 2: {}", shared_data2.value);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
