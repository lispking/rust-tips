fn main() {
    let my_bubble_tea = String::from("珍珠奶茶");
    let friend_bubble_tea = my_bubble_tea;
    // println!("我想喝我的奶茶：{}", my_bubble_tea); // ^^^^^^^^^^^^^ value borrowed here after move
    println!("送给朋友的：{}", friend_bubble_tea);
}
