fn main() {
    let numbers = [1, 2, 3, 4, 5];

    let even_numbers = numbers
        .iter()
        .filter(|&num| num % 2 == 0)
        .collect::<Vec<_>>();

    for num in even_numbers {
        println!("{} 是偶数", num);
    }
}
