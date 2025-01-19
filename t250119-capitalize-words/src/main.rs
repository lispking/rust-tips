fn capitalize_words(s: &str) -> String {
    let mut result = String::new();
    let mut words = s.split_whitespace();
    if let Some(first_word) = words.next() {
        result.push_str(&first_word[0..1].to_uppercase());
        result.push_str(&first_word[1..]);
    }
    for word in words {
        result.push(' ');
        result.push_str(&word[0..1].to_uppercase());
        result.push_str(&word[1..]);
    }
    result
}

fn capitalize_wordsv2(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            word.chars().next().map_or_else(String::new, |c| {
                format!("{}{}", c.to_uppercase(), &word[1..])
            })
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    println!("{}", capitalize_words("hello, world!"));
    println!("{}", capitalize_wordsv2("hello, world!"));
}
