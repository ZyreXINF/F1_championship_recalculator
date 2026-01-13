use crate::model::point_system::PointSystem;
use std::fs;

const MODERN_CONFIG: &str = include_str!("../default_configs/modern.toml");
const CLASSIC_CONFIG: &str = include_str!("../default_configs/classic.toml");
const LEGACY_CONFIG: &str = include_str!("../default_configs/legacy.toml");

pub fn parse_config(path: &str) -> PointSystem {
    let content = fs::read_to_string(path).expect("Could not read config file");
    toml::from_str(&content).expect("Failed to parse TOML")
}
pub fn parse_embedded_config(name: &str) -> PointSystem {
    let content = match name {
        "modern" => MODERN_CONFIG,
        "classic" => CLASSIC_CONFIG,
        "legacy" => LEGACY_CONFIG,
        _ => MODERN_CONFIG,
    };

    toml::from_str(content).expect("Failed to parse embedded TOML")
}
