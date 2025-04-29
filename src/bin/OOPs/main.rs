// main.rs
mod models {
  pub mod person;
  pub mod customer;
  pub mod driver;
  pub mod vehicle;
  pub mod ride;
}
mod service {
  pub mod ride_service;
  pub mod uber_service;
}

use models::{customer::Customer, driver::Driver, vehicle::Vehicle};
use service::{ride_service::RideService, uber_service::UberService};

fn main() {
  let mut customer = Customer::new("Alice", 100.0);
  let vehicle = Vehicle::new("ABC123", 4);
  let mut driver = Driver::new("Bob", vehicle);

  let uber = UberService;
  let ride = uber.request_ride(&mut customer, &mut driver, "Home", "Airport", 25.0);

  ride.print_summary();

  println!("Customer Wallet: ${:.2}", customer.wallet);
  println!("Driver Wallet: ${:.2}", driver.wallet);
}
