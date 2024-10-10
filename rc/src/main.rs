use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    pub name: String,
    pub age: u8,
}

fn main() {
    let p1 = Person {
        name: "John".to_owned(),
        age: 18,
    };
    let rc1 = Rc::new(p1);
    let rc2 = rc1.clone();
    let rc3 = rc2.clone();
    let wk = Rc::downgrade(&rc1);
    println!("rc1 count: {}", Rc::strong_count(&rc1));
    drop(rc1);
    println!("rc2 count: {}", Rc::strong_count(&rc2));
    // 由于 rc1 已经被释放， rc2 和 rc3 现在还持有对 Person 的引用

    drop(rc2);
    drop(rc3);
    // 打印 weak 引用是否有效
    if let Some(_strong) = wk.upgrade() {
        println!("Weak reference is still valid.");
    } else {
        println!("Weak reference is no longer valid.");
    }
}
