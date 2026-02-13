#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: T,
    point_name: U
}
fn main() {
    let point1 = Point { x: 5, y: 10, point_name: "a" };
    // let point2 = Point { x: 1.4, y: 2.4 };    
    println!("int point: {:?}", point1);
}