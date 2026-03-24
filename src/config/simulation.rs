use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct BodyConfig {
    pub name: String,
    pub mass: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub color: String,
}

#[derive(Deserialize)]
pub struct SimulationConfig {
    pub bodies: Vec<BodyConfig>,
    pub dt: f64,
    pub steps: usize,
}

pub fn load_config(path: &str) -> SimulationConfig {
    let data = fs::read_to_string(path)
        .expect("Failed to read config file");

    serde_json::from_str(&data)
        .expect("Failed to parse JSON")
}