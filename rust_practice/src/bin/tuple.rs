/*
tuple -> (), uses parenthesis
 -> a type of record
-> store data anonymously, no need to name fields
-> useful to return pairs of data from fn
-> can be destructured easily into variable
*/

// demo
// fn main() {
//   let coord = (2, 3);
//   println!("{} {}", coord.0, coord.1);

//   let (x, y) = (2, 3); // destructuring
//   println!("{} {}", x, y);

//   let (name, age) = ("manish", 21);
//   println!("name: {}, age: {}", name, age)
// }

//exercise

fn ret(a: (i32, i32)) -> (i32, i32) {
  let (x, y) = a;
  (x, y)
}
fn main() {
  let a = (3, 5);
  let (x, y) = ret(a);
  if y > 5 {
    println!("greater");
  } else if y < 5 {
    println!("smaller");
  } else {
    println!("equal");
  }
}
