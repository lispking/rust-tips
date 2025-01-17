// use std::fs::File;
// use std::io::{self, Read};

// fn main() -> io::Result<()> {
//     let mut file = File::open("README.md")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     println!("{}", contents);
//     Ok(())
// }

use std::fs;

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("README.md")?;
    println!("{}", contents);
    Ok(())
}
