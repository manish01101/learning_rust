

mod my_module {
  fn print() {
    println!("this is private fn inside module");
  }
  pub fn printPb() {
    println!("this is public fn inside module");
  }
}

fn main() {
  my_module::printPb();
}