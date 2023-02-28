pub mod error;
use clap::Parser;
use config::{Config, File};
use std::collections::HashMap;
use crate::error::EnvTestError;

use std::time;
use std::thread;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   config: String,

   #[arg(short, long)]
   idx: u8,
}

#[tokio::main]
async fn main() -> Result<(), EnvTestError> {
    let args = Args::parse();

    let config_builder = Config::builder()
        .add_source(File::with_name(args.config.as_str()))
        .build()
        .map_err(|_| EnvTestError::ConfigError)?;

    let host_config = config_builder
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| EnvTestError::ConfigError)?;

    let log_file = format!("{}env_client{}.log",
                           &host_config["log-dir"][0],
                           args.idx);
    let raw_log_file = format!("{}env_client{}.log.raw",
                               &host_config["log-dir"][0],
                               args.idx);
    println!("This is envtest client#{} logging to {}.", args.idx, log_file);

    let mut log = std::fs::File::create(log_file)
        .map_err(|_| EnvTestError::FileOpError)?;
    let mut raw_log = std::fs::File::create(raw_log_file)
        .map_err(|_| EnvTestError::FileOpError)?;

    loop {
        let num_servers = host_config["server-hosts"].len();
        for idx in 1..num_servers {
            let host = &host_config["server-hosts"][idx];
            let port = &host_config["server-ports"][idx];

            let sent = SystemTime::now();
            let addr = format!("{}:{}", host, port);
            let mut stream = TcpStream::connect(addr)
                .map_err(|_| EnvTestError::TcpConnError)?;

            let mut rx_bytes = [0u8; 64];
            stream.read(&mut rx_bytes)
                .map_err(|_| EnvTestError::TcpReadError)?;

            raw_log.write_all(&rx_bytes)
                .map_err(|_| EnvTestError::FileOpError)?;

            let entry = format!("{:?}\n", sent.elapsed().unwrap());
            log.write_all(&entry.as_bytes())
                .map_err(|_| EnvTestError::FileOpError)?;

            thread::sleep(time::Duration::from_millis(50));
        }
    };
}
