fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // 创建一个切片
    let slice = &numbers[1..4];

    // 遍历切片
    for num in slice {
        println!("{}", num);
    }

    // 修改切片元素（前提是原始数组是可变的）
    let mut mutable_numbers = [1, 2, 3, 4, 5];
    println!("修改前：{:?}", mutable_numbers);
    let mutable_slice = &mut mutable_numbers[1..4];
    mutable_slice[0] = 10;
    println!("修改后：{:?}", mutable_numbers);
}
