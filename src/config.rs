use config::{Config, File};
use crate::cli::TargetType;
use std::collections::HashMap;
use crate::error::OracleError;

pub fn read_config(target: &TargetType) -> Result<HashMap<String, Vec<String>>, OracleError> {
    let target_config = match target {
        TargetType::EnvTest => "config/envtest",
        TargetType::EnvTestGeo => "config/envtest_geo",
        TargetType::HotStuff => "config/hotstuff",
        TargetType::HotStuffBumped => "config/hotstuff_bumped",
        TargetType::Pompe => "config/pompe",
        TargetType::PompeBumped => "config/pompe_bumped",
        TargetType::PompeUnbiasBumped => "config/pompe_unbias_bumped",
        TargetType::LargeHotStuffBumped => "config/large_hotstuff_bumped",
        TargetType::LargeThemisBumped => "config/large_themis_bumped",
        TargetType::LargePompeBumped => "config/large_pompe_bumped",
        TargetType::LargePompeUnbiasBumped => "config/large_pompe_unbias_bumped",
        _ => Err( OracleError::ConfigError )?
    };

    let config = Config::builder()
        .add_source(File::with_name(target_config))
        .build()
        .map_err(|_| OracleError::ConfigError)?;

    return config
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
