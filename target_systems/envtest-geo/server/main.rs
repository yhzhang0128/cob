pub mod error;
use clap::Parser;
use config::{Config, File};
use std::collections::HashMap;
use crate::error::EnvTestError;

use std::{
    io::prelude::*,
    net::TcpListener,
    time::SystemTime,
};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    idx: usize,
    #[arg(long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), EnvTestError> {
    let args = Args::parse();
    let idx = args.idx;

    let config_builder = Config::builder()
        .add_source(File::with_name(args.config.as_str()))
        .build()
        .map_err(|_| EnvTestError::ConfigError)?;

    let config = config_builder
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| EnvTestError::ConfigError)?;

    let host = &config["server-hosts"][idx];
    let port = &config["server-ports"][idx];
    let bind = format!("{}:{}", host, port);
    println!("This is envtest server#{}, port={}.", idx, port);

    let listener = TcpListener::bind(bind).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let sys_time = SystemTime::now();
        let response = format!("{:?}, server{}\n", sys_time, idx);

        stream.write_all(response.as_bytes()).unwrap();
    }

    Ok(())
}

