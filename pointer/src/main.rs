use std::mem;

#[derive(Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

fn main() {
    let pt = Point { x: 1, y: 2 };
    let x = 0u64;
    let ref_x = &x;
    let ref_pt = &pt;
    // {:?} 格式说明符会打印引用所指向的值
    println!("ref_x: {:?}", ref_x);
    println!(":p ref_x: {:p}", ref_x);
    println!(":p x: {:p}", &x);
    println!("ref_pt: {:?}", ref_pt);

    let box_pt = Box::new(Point { x: 1, y: 2 });
    println!("size of pointer of box_pt: {:?}", mem::size_of_val(&box_pt));
    println!("size of box_pt: {:?}", mem::size_of_val(&*box_pt));
}
