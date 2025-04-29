// models/vehicle.rs
pub struct Vehicle {
  pub license_plate: String,
  pub capacity: u8,
}

impl Vehicle {
  pub fn new(license_plate: &str, capacity: u8) -> Self {
      Self {
          license_plate: license_plate.to_string(),
          capacity,
      }
  }
}
