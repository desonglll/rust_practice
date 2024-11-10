use std::collections::BTreeMap;
fn main() {
    let mut map = BTreeMap::<String, String>::new();
    let names = ["mike", "john", "doe", "alice"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let full_names = ["mike shinoda", "john steve", "doe gallegher", "alice bob"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut enumerate = names
        .iter()
        .zip(full_names.iter())
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect::<Vec<(String, String)>>();

    println!("Vec:");
    enumerate
        .iter_mut()
        .map(|(name, value)| {
            *value = value.to_uppercase();
            (name, value)
        })
        .for_each(|(name, value)| {
            map.insert(name.clone(), value.clone());
            println!("name: {:?}, \tvalue: {:?}", name, value);
        });

    println!("BTreeMap:");
    map.iter_mut().for_each(|(name, value)| {
        *value = value.to_lowercase();
        println!("name: {:?}, \tvalue: {:?}", name, value);
    });
}
