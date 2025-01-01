fn print_string(value: &String) {
    println!("Borrowed: {}", value);
}

fn take_ownership(value: String) {
    // 这里拥有了传入值的所有权
    println!("Received: {}", value);
}

fn main() {
    let happy_new_year = String::from("Hello, 2025!");
    print_string(&happy_new_year);
    // 所有权未转移，可以继续使用 happy_new_year

    take_ownership(happy_new_year); 
    // 此时 happy_new_year 的所有权已转移，不能再使用
}