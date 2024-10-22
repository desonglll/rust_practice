use std::error::Error;

fn h(i: i32) -> Result<i32, String> {
    if i > 0 {
        Ok(i + 10)
    } else {
        Err("Please enter a positive integer".into())
    }
}

#[allow(unused)]
fn foo(input: Option<i32>) -> Option<i32> {
    let input = input?;
    if input < 0 {
        return None;
    }
    Some(input)
}
fn main() -> Result<(), Box<dyn Error>> {
    let i = h(2)?;
    println!("i: {}", i);
    Ok(())
}
