use config::{Config, File};
use std::collections::HashMap;
use crate::error::OracleError;

#[derive(Debug)]
pub enum TargetType {
    EnvTest,
    HotStuff,
    Pompe,
    Unknown,
}

pub fn read_host_config() -> Result<HashMap<String, Vec<String>>, OracleError> {
    let host_config = Config::builder()
        .add_source(File::with_name("config/host"))
        .build()
        .map_err(|_| OracleError::ConfigError)?;

    return host_config
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| OracleError::ConfigError);
}

pub fn read_latency_config() -> Result<HashMap<String, Vec<i32>>, OracleError> {
    // Read latency configuration
    let latency_config = Config::builder()
        .add_source(File::with_name("config/latency"))
        .build()
        .map_err(|_| OracleError::ConfigError)?;

    let latencies = latency_config
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| OracleError::ConfigError)?;

    // Parse strings into i32 integers
    let mut latency_matrix : HashMap<String, Vec<i32>> = HashMap::new();
    for location in &latencies["locations"] {
        let mut tmp : Vec<i32> = Vec::new();
        for latency in &latencies[location] {
            let number = latency.parse::<i32>()
                .map_err(|_| OracleError::ConfigError)?;
            tmp.push(number);
        }
        latency_matrix.insert(location.to_string(), tmp);
    }
    return Ok(latency_matrix);
}
