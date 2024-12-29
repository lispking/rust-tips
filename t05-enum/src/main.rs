enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("这是红色"),
        Color::Green => println!("这是绿色"),
        Color::Blue => println!("这是蓝色"),
    }
}

fn main() {
    let my_color = Color::Green;
    print_color(my_color);
}