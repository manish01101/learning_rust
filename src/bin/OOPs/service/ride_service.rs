// service/ride_service.rs
use crate::models::{customer::Customer, driver::Driver, ride::Ride};

pub trait RideService {
    fn request_ride(&self, customer: &mut Customer, driver: &mut Driver, from: &str, to: &str, fare: f64) -> Ride;
}
