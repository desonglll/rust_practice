fn main() {
    let person = init_vec().unwrap();
    let name = get_name(person, 1)
        .map(|mut new_person| {
            new_person.name = format!("new {}", new_person.name);
            new_person
        })
        .unwrap();
    println!("Name: {:?}", name);
}
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Person {
    pub name: String,
    pub age: u8,
}

fn init_vec() -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let mut person = Vec::<Person>::new();
    for i in 0..10 {
        let insert_person = Person {
            name: format!("person {}", i),
            age: i + 10,
        };
        person.push(insert_person)
    }
    Ok(person)
}

fn get_name(person: Vec<Person>, idx: usize) -> Result<Person, Box<dyn std::error::Error>> {
    Ok(person[idx].clone())
}
