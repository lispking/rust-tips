fn add_n<F>(func: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    move |x| func(x)
}

fn main() {
    let add_five = add_n(|x| x + 5);
    let result = add_five(10);
    println!("result: {}", result);
}
