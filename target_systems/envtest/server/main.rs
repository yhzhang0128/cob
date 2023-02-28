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
   #[arg(short, long)]
   config: String,

   #[arg(short, long)]
   idx: usize,
}

#[tokio::main]
async fn main() -> Result<(), EnvTestError> {
    let args = Args::parse();
    let idx = args.idx;

    let config_builder = Config::builder()
        .add_source(File::with_name(args.config.as_str()))
        .build()
        .map_err(|_| EnvTestError::ConfigError)?;

    let host_config = config_builder
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| EnvTestError::ConfigError)?;

    let port = &host_config["server-ports"][idx];
    let bind = format!("127.0.0.1:{}", port);
    println!("This is envtest server#{}, port={}.", idx, port);

    let listener = TcpListener::bind(bind).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let sys_time = SystemTime::now();
        let response = format!("server{}: {:?}\n", idx, sys_time);

        stream.write_all(response.as_bytes()).unwrap();
    }

    Ok(())
}

