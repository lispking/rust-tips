fn check_number(num: i32) {
    match num {
        0 => println!("这是零"),
        1 => println!("这是一"),
        2 => println!("这是二"),
        _ => println!("这是其他数字"),
    }
}

fn main() {
    check_number(0);
}
