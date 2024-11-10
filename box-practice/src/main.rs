fn main() {
    let a: i32 = 123;
    let box_a: Box<i32> = Box::new(123);
    println!("a: {}", a);
    println!("box_a: {}", box_a);
    println!("pointer a: {:?}", &a as *const i32);
    println!("pointer box_a: {:p}", box_a);
}
