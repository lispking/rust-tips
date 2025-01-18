fn is_all_digits(s: &str) -> bool {
    for char in s.chars() {
        if !char.is_digit(10) {
            return false;
        }
    }
    true
}

fn is_all_digits_v2(s: &str) -> bool {
    s.chars().all(|char| char.is_digit(10))
}

fn main() {
    println!("is_all_digits: {}", is_all_digits("123456"));
    println!("is_all_digits_v2: {}", is_all_digits_v2("123456"));
}
