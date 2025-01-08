trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("汪汪汪");
    }
}

fn main() {
    // 使用 Box<dyn Animal> 存储动态类型
    let animal: Box<dyn Animal> = Box::new(Dog {});

    // 调用动态类型的方法
    animal.make_sound();
}
