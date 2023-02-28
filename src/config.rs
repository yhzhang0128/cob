use config::{Config, File};
use std::collections::HashMap;
use crate::error::OracleError;

pub fn read_host_config() -> Result<HashMap<String, Vec<String>>, OracleError> {
    let host_config = Config::builder()
        .add_source(File::with_name("config/host"))
        .build()
        .map_err(|_| OracleError::ConfigError)?;

    return host_config
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| OracleError::ConfigError);
}

pub fn read_latency_config() -> Result<HashMap<String, Vec<String>>, OracleError> {
    // Read latency configuration
    let latency_config = Config::builder()
        .add_source(File::with_name("config/latency"))
        .build()
        .map_err(|_| OracleError::ConfigError)?;

    return latency_config
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| OracleError::ConfigError);
}
