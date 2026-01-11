use crate::model::point_system::PointSystem;
use std::fs;

pub fn parse_config(path: &str) -> PointSystem {
    let content = fs::read_to_string(path).expect("Could not read config file");
    toml::from_str(&content).expect("Failed to parse TOML")
}
