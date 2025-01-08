fn apply_operation<F>(numbers: &[i32], operation: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    numbers.iter().map(|&num| operation(num)).collect()
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = apply_operation(&numbers, double);
    println!("{:?}", result);
}
