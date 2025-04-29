// service/uber_service.rs
use crate::service::ride_service::RideService;
use crate::models::{customer::Customer, driver::Driver, ride::Ride};

pub struct UberService;

impl RideService for UberService {
    fn request_ride(&self, customer: &mut Customer, driver: &mut Driver, from: &str, to: &str, fare: f64) -> Ride {
        customer.pay(fare);
        driver.receive_payment(fare);
        Ride::new(from, to, fare, customer, driver)
    }
}
