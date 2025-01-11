macro_rules! create_fn {
    ($name:ident, $value:expr) => {
        fn $name() -> i32 {
            $value
        }
    };
}

create_fn!(get_five, 5);
create_fn!(get_ten, 10);

fn main() {
    println!("{}", get_five()); // 输出 5
    println!("{}", get_ten()); // 输出 10
}
