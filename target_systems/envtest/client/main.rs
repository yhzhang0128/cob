pub mod error;
use clap::Parser;
use config::{Config, File};
use std::collections::HashMap;
use crate::error::EnvTestError;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   config: String,

   #[arg(short, long)]
   log: String,

   #[arg(short, long)]
   idx: u8,
}

#[tokio::main]
async fn main() -> Result<(), EnvTestError> {
    let args = Args::parse();

    let config_builder = Config::builder()
        .add_source(File::with_name("config/host"))
        .build()
        .map_err(|_| EnvTestError::ConfigError)?;

    let host_config = config_builder
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| EnvTestError::ConfigError)?;

    let log_file = format!("{}env_client.log", &host_config["log-dir"][0]);
    println!("This is envtest client#{} logging to {}.", args.idx, log_file);

    let mut file = std::fs::File::create(log_file)
        .map_err(|_| EnvTestError::FileOpError)?;
    file.write_all(b"This is the log of an envtest client.\n")
        .map_err(|_| EnvTestError::FileOpError)?;

    loop {};
}
