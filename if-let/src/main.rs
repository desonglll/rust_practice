#[allow(unused)]
enum Gender {
    Man(String),
    Woman(String),
    Unknown(String),
}
fn gender(s: String) -> Gender {
    match s.as_str() {
        "man" => Gender::Man("this is man".to_string()),
        "woman" => Gender::Woman("this is woman".to_string()),
        _ => Gender::Unknown("this is unknown".to_string()),
    }
}
fn hello(s: String) -> Result<String, Box<dyn std::error::Error>> {
    match s.as_str() {
        "world" => Ok(s.to_string()),
        _ => Err("world must be a string".into()),
    }
}

fn main() {
    let world = String::from("world");
    if let Ok(s) = hello(world) {
        println!("world: {}", s);
    }
    let man = String::from("man");
    if let Gender::Man(man) = gender(man) {
        println!("gender: {}", man);
    }
}
