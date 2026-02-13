trait Draw {
    fn draw(&self); // object safe
    // fn create()-> self; // not object safe
}
struct Circle;
struct Square;

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}
impl Draw for Square {
    fn draw(&self) {
        println!("Drawing Square");
    }
}

fn draw_object(shape: &dyn Draw) {
    shape.draw();
}
fn main() {
    let circle = Circle;
    draw_object(&circle);

    let square = Square;
    draw_object(&square);
}
