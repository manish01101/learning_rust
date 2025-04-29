// models/driver.rs
use crate::models::person::Person;
use crate::models::vehicle::Vehicle;

pub struct Driver {
    name: String,
    pub wallet: f64,
    pub vehicle: Vehicle,
}

impl Driver {
    pub fn new(name: &str, vehicle: Vehicle) -> Self {
        Self {
            name: name.to_string(),
            wallet: 0.0,
            vehicle,
        }
    }

    pub fn receive_payment(&mut self, amount: f64) {
        self.wallet += amount;
    }
}

impl Person for Driver {
    fn get_name(&self) -> &str {
        &self.name
    }
}
