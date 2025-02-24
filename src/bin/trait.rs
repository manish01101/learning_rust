trait Printable {
    fn print(&self);
}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle area : {} x {}", self.length, self.width);
    }
}

fn main() {
  let rect1 = Rectangle{width:4, length:5};
  rect1.print();
}