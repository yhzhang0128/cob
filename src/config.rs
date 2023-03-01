use config::{Config, File};
use crate::cli::TargetType;
use std::collections::HashMap;
use crate::error::OracleError;

pub fn read_host_config(target: &TargetType) -> Result<HashMap<String, Vec<String>>, OracleError> {
    let target_config = match target {
        TargetType::EnvTest => "config/envtest",
        TargetType::HotStuff => "config/hotstuff",
        TargetType::Pompe => "config/pompe",
        _ => Err( OracleError::ConfigError )?
    };

    let host_config = Config::builder()
        .add_source(File::with_name("config/host"))
        .add_source(File::with_name(target_config))
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
