use std::ops::Deref;

#[derive(Debug)]
#[allow(unused)]
struct Point {
    x: i32,
    y: i32,
    message: String,
}

impl Deref for Point {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[derive(Debug)]
struct MyError;

fn get_point(is: bool) -> Result<Point, MyError> {
    if is {
        Ok(Point {
            x: 1,
            y: 2,
            message: String::from("hello"),
        })
    } else {
        Err(MyError)
    }
}

pub fn main() {
    let a = 10;
    println!("a: {}", a);
    let mut binding = get_point(true);
    let result = binding.as_deref().unwrap();
    println!("result: {:?}", result);
    let mut_result = binding.as_mut().unwrap();
    mut_result.message = String::from("world");
    println!("mut_result: {:?}", mut_result);
}
