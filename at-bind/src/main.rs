fn main() {
    let coordinates = (2, 3);

    let point @ (x, _) = coordinates;
    println!("The x-coordinate is {}", x);
    println!("The point is at {:?}", point);
}
