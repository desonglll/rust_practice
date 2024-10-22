fn h(i: i32) -> Result<i32, String> {
    if i > 0 {
        Ok(i + 10)
    } else {
        Err("Please enter a positive integer".into())
    }
}
fn main() {
    let Ok(i) = h(0) else {
        let error = h(0).unwrap_err();
        println!("Error: {}", error);
        return;
    };
    println!("i: {:?}", i);
}
