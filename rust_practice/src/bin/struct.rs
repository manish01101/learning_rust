enum Flavor {
  Sparkling,
  Sweet,
  Fruity,
}

struct Drink {
  flavor: Flavor,
  fluid_oz: f64,
}
fn print_drink(drink: Drink) {
  match drink.flavor {
    Flavor::Sparkling => println!("sparkling"),
    Flavor::Sweet => println!("sweet"),
    Flavor::Fruity => println!("fruity"),
  }
  println!("ox: {}", drink.fluid_oz);
}

fn main() {
  let sweet = Drink {
    flavor: Flavor::Sweet,
    fluid_oz: 6.0
  };
  print_drink(sweet);

  let fruity = Drink {
    flavor: Flavor::Fruity,
    fluid_oz: 6.0
  };
  print_drink(fruity);
}