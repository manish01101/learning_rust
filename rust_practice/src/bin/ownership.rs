/*
rust uses "ownership" for memory management
-> the "owner" of data must clean up the memory
-> this occurs automatically at the end of the scope
->ownership: either move or borrow
->default behavior is to "move" memory to a new owner
-> use an "&" to allow code to 'borrow' memory
*/

// move
// enum Light {
//   Bright, Dull,
// }
// fn display_light(light: Light) {
//   match light {
//     Light::Bright => println!("bright"),
//     Light::Dull => println!("dull"),
//   }
// }
// fn main() {
//   let dull = Light::Dull;
//   display_light(dull); // dull is transferred to fn call at line 17
//   // now there is no dull variable in main fn
//   display_light(dull); // so in line 18 there is no dull variable, therefore, throw err
// }



// borrow
// enum Light {
//   Bright, Dull,
// }
// fn display_light(light: &Light) {
//   match light {
//     Light::Bright => println!("bright"),
//     Light::Dull => println!("dull"),
//   }
// }
// fn main() {
//   let dull = Light::Dull;
//   display_light(&dull); 
//   display_light(&dull);
// }



//exercise
struct GroceryItem {
  quantity: i32,
  id: i32
}
fn display_quantity(item: &GroceryItem) {
  println!("quantity: {}", item.quantity);
}
fn display_id(item: &GroceryItem) {
  println!("id: {}", item.id);
}
fn main() {
  let my_item = GroceryItem {
    quantity: 3,
    id: 99,
  };
  display_quantity(&my_item);
  display_id(&my_item);

}