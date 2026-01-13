use std::collections::HashMap;
use crate::model::driver::Driver;

#[derive(Debug)]
pub struct RaceResults{
    pub saved: HashMap<u16, Vec<Driver>>
}
impl RaceResults {
    pub fn new() -> Self {
        Self{
            saved: HashMap::new()
        }
    }
}