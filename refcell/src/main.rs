use std::cell::RefCell;
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
    let rc = RefCell::new(p1);
    // let _b1 = rc.borrow();
    let mut b2 = rc.borrow_mut();
    b2.name = "Mike".to_owned();
}
