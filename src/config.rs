use config::{Config, File};
use std::collections::HashMap;
use crate::error::OracleError;

fn read_machine_config() -> Result<HashMap<String, Vec<String>>, OracleError> {
    let machine_config = Config::builder()
        .add_source(File::with_name("config/machine"))
        .build()
        .map_err(|_| OracleError::ConfigError)?;
    return machine_config
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| OracleError::ConfigError);
}

pub fn config_latency() -> Result<(), OracleError> {
    // Reading latency configuration
    let latency_config = Config::builder()
        .add_source(File::with_name("config/latency"))
        .build()
        .map_err(|_| OracleError::ConfigError)?;

    let latencies = latency_config
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| OracleError::ConfigError)?;

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
    println!("# Latency matrix:");
    println!("{:?}", latency_matrix);

    // Reading machine configuration
    let machines = read_machine_config()?;
    println!("# Machine configuration:");
    println!("{:?}", machines);

    Ok(())
}
