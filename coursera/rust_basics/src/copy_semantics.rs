/*
copy semantics means duplicating data rather than moving it when assigning to another variable or passing to a fn, this keeps the original data and the copy separate, allowing for safe and flexible manipulation.

basics types like int, float, char and bool use copy semantics by default
*/
#[derive(Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let point1 = Point { x: 5, y: 10 };
    let point2 = point1; // without copy trait data will move, in complex data type
    println!("p1 x: {}, p1 y: {}", point1.x, point1.y);
    println!("p2 x: {}, p2 y: {}", point2.x, point2.y);
}
