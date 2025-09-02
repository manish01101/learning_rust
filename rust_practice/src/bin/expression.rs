/*
rust is expr based lang
most things are evaluated and return some value
->expr val coalesce to a single point , can be used for nesting logic */

// demo
// enum Access {
//   Admin, Manager, User, Guest
// }


// fn main() {
//   let access_level = Access::Guest;
//   let can_access_file = match access_level {
//     Access::Admin => true,
//     _ => false,
//   };
//   println!("{}",can_access_file)
// }

// exercise
fn print_message(b: bool) {
  match b {
    true => println!("greater"),
    false => println!("small"),
  }
}
fn main() {
  let val = 100;
  let is_gre = val > 100;
  print_message(is_gre)
}
