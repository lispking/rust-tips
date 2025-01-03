use std::num::ParseIntError;

fn double_and_convert(s: &str) -> Result<i32, ParseIntError> {
    let num = s.parse::<i32>()?;
    Ok(num * 2)
}

fn main() {
    match double_and_convert("10") {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
