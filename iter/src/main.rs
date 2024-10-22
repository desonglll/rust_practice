fn main() {
    let a: Option<[i32; 5]> = Some([1, 2, 3, 4, 5]);
    for i in a.iter() {
        println!("{:?}", i);
    }
    println!("Hello, world!");
}
