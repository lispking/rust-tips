use std::fs::File;
use std::io::{Error, Read};

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("my_file.txt") {
        Ok(contents) => println!("文件内容: {}", contents),
        Err(e) => println!("错误: {}", e),
    }
}
