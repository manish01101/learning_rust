// models/customer.rs
use crate::models::person::Person;

pub struct Customer {
    name: String,
    pub wallet: f64,
}

impl Customer {
    pub fn new(name: &str, wallet: f64) -> Self {
        Self {
            name: name.to_string(),
            wallet,
        }
    }

    pub fn pay(&mut self, amount: f64) {
        self.wallet -= amount;
    }
}

impl Person for Customer {
    fn get_name(&self) -> &str {
        &self.name
    }
}
