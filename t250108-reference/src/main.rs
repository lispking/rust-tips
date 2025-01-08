fn modify_value(num: &mut i32) {
    *num += 1;
}

fn double_value(num: &i32) -> i32 {
    *num * 2
}

fn main() {
    let mut number = 5;
    modify_value(&mut number);
    println!("Modified number: {}", number);

    let num = 3;
    println!("double value: {}", double_value(&num));
}
