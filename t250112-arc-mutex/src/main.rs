use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个共享的、被互斥锁保护的空Vec
    let shared_vec = Arc::new(Mutex::new(Vec::<i32>::new()));
    let mut handles = vec![];

    // 创建10个线程
    for _ in 0..10 {
        let shared_vec = Arc::clone(&shared_vec);
        handles.push(thread::spawn(move || {
            shared_vec.lock().unwrap().push(1);
        }));
    }

    // 等待所有线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }

    // 获取最终的共享Vec，并打印其长度（也就是元素个数）
    let final_vec = shared_vec.lock().unwrap();
    println!("The length of the shared vector is {}", final_vec.len());
}
