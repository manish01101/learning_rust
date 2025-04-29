// models/ride.rs
use crate::models::{customer::Customer, driver::Driver};

use super::person::Person;

pub struct Ride {
    pub from: String,
    pub to: String,
    pub fare: f64,
    pub customer_name: String,
    pub driver_name: String,
}

impl Ride {
    pub fn new(from: &str, to: &str, fare: f64, customer: &Customer, driver: &Driver) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            fare,
            customer_name: customer.get_name().to_string(),
            driver_name: driver.get_name().to_string(),
        }
    }

    pub fn print_summary(&self) {
        println!("Ride from {} to {} by {} for {}. Fare: ${:.2}", self.from, self.to, self.driver_name, self.customer_name, self.fare);
    }
}
