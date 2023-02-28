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

use std::sync::Arc;
use signal_hook::flag;
use std::sync::atomic::{AtomicBool, Ordering};

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

    let dir = &host_config["log-dir"][0];
    let log_file = format!("{}env_client{}_rtt.log", dir, args.idx);
    let latency_file = format!("{}latency{}.log", dir, args.idx);
    let mut log = std::fs::File::create(&log_file)
        .map_err(|_| EnvTestError::FileOpError)?;
    let mut latency = std::fs::File::create(&latency_file)
        .map_err(|_| EnvTestError::FileOpError)?;
    println!("This is envtest client#{} logging to {}.", args.idx, log_file);
    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGTERM kill signal
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))
        .map_err(|_| EnvTestError::SigTermHandlerError)?;

    while !term.load(Ordering::Relaxed) {
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

            let entry = format!("{:?}\n", sent.elapsed().unwrap());
            log.write_all(&entry.as_bytes())
                .map_err(|_| EnvTestError::FileOpError)?;

            thread::sleep(time::Duration::from_millis(50));
        }
    }

    // Client terminated by signal, print latency info
    let row1 = format!("Write to file after terminated\n");
    latency.write_all(&row1.as_bytes())
        .map_err(|_| EnvTestError::FileOpError)?;

    Ok(())
}
