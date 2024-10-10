use std::borrow::Cow;

#[allow(dead_code)]
fn process(v: &mut Cow<'_, [i32]>) {
    println!("Before");
    for (k, v) in v.to_mut().iter().enumerate() {
        println!("index: {}, value: {}", k, v);
    }
    println!("After");
    for (k, v) in v.to_mut().iter_mut().enumerate() {
        println!("index: {}, value: {}", k, v.pow(2));
    }
}

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut list = Cow::from(&v);
    assert_eq!(list, Cow::Borrowed(&vec![1, 2, 3, 4, 5]));
    // process(&mut cow);
    let mut_list = list.to_mut();
    mut_list.push(6);
    assert_eq!(list, Cow::Owned::<Vec<i32>>(vec![1, 2, 3, 4, 5, 6]));
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}
