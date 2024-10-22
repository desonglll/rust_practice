fn main() {
    let vec = vec![0, 1, 2, 3];
    vec.iter().for_each(|v| println!("{}", v));

    for v in &vec {
        println!("{}", v);
    }
}
