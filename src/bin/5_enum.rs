enum IpAddrKind {
  V4,
  V6,
}
struct IpAddr {
  kind: IpAddrKind,
  address: String,
}
enum IpAddress {
  V4(String),
  V6(String),
}
enum IpAddress1 {
  V4(u8, u8, u8, u8),
  V6(String),
}
struct Ipv4Addr {}
struct Ipv6Addr {}
enum IpAddress2 {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
/* DEFINING METHODS ON ENUM */
impl Message {
  fn call(&self) {
      println!("fn called\n");
  }
}
/* OPTION ENUM */
// enum Option<T> {
//     None,
//     Some(T),
// }

/* PATTERNS THAT BIND TO VALUES */
#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn main() {
  let home = IpAddr {
      kind: IpAddrKind::V4,
      address: String::from("127.0.0.1"),
  };
  let loopback = IpAddr {
      kind: IpAddrKind::V6,
      address: String::from("::1"),
  };

  /* PUT DATA DIRECTLY INTO EACH ENUM VARIABLE */
  let home1 = IpAddress::V4(String::from("127.0.0.1"));
  let loopback1 = IpAddress::V6(String::from("::1"));

  /* EACH VAL CAN HAVE DIFFERENT TYPES & AMOUNTS OF ASSOCIALTED DATA */
  let home2 = IpAddress1::V4(127, 0, 0, 1);
  let loopback2 = IpAddress1::V6(String::from("::1"));

  /* using method on enum */
  let m = Message::Write(String::from("hello manish"));
  m.call();

  /* option enum */
  let some_number = Some(5); // need not to write Option:: as option is included in prelude
  let absent_number: Option<i32> = None;

  /* patterns that bind to values */
  fn value_in_cents(coin: Coin) -> u8 {
      match coin {
          Coin::Penny => 1,
          Coin::Nickel => 5,
          Coin::Dime => 10,
          Coin::Quarter(state) => {
              println!("State quarter from {:?}!", state);
              25
          }
      }
  }

  /* matching with Option<T> */
  fn plus_one(x: Option<i32>) -> Option<i32> {
      match x {
          None => None,
          Some(i) => Some(i + 1),
      }
  }
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  dbg!("{}", &five);

  /* -- Catch-All patterns and the _ Placeholder */
  let dice_roll = 9;
  match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      other => move_player(other), // catch all other val and want to use that other val
      // _ => println!("catched non used val"), // catch all other val and dont want to use that val
  }
  fn add_fancy_hat() {}
  fn remove_fancy_hat() {}
  fn move_player(num_spaces: u8) {}

  /* --- CONCISE CONTROL FLOW WITH "if let" */
  // with match pattern
  let config_max = Some(3u8);
  match config_max {
      Some(max) => println!("the max is {max}"),
      _ => (), // here we have to use catch-all as match is exhaustive
  }
  // with "if let"
  if let Some(max) = config_max {
      println!("the max is {max}");
  }
  // with "if let, else" => similar as match pattern
  if let Some(max) = config_max {
      println!("the max is {max}");
  } else {
      println!("no max value found");
  }
}
