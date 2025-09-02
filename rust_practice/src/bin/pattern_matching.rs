enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn cal_area(shape: Shape) -> f64 {
    // pattern matching
    let ans = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(s) => s * s,
        Shape::Rectangle(l, b) => l * b,
    };
    return ans;
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(5.0);
    let rec = Shape::Rectangle(4.0, 4.0);

    println!("cir: {}", cal_area(circle));
    println!("sq: {}", cal_area(square));
    println!("rec: {}", cal_area(rec));
}
