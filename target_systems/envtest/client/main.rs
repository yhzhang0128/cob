pub mod error;
use clap::Parser;
use colored::Colorize;
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
    #[arg(long)]
    host: String,
    #[arg(long)]
    latency: u64,
    #[arg(long)]
    idx: u32,
    #[arg(long)]
    serveridx: usize,
}

#[tokio::main]
async fn main() -> Result<(), EnvTestError> {
    let args = Args::parse();
    let config_builder = Config::builder()
        .add_source(File::with_name(args.host.as_str()))
        .build()
        .map_err(|_| EnvTestError::ConfigError)?;
    let host_config = config_builder
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| EnvTestError::ConfigError)?;

    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGTERM kill signal
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))
        .map_err(|_| EnvTestError::SigTermHandlerError)?;

    // Execute TCP client logic
    let mut latencies: Vec<u128> = vec![];
    match tcp_client(term, &mut latencies, &host_config) {
        Ok(()) => {}
        Err(_err) => { /*TCP error can occur when experiment terminates*/ }
    }

    // Client terminated by signal, print latency info
    let mut sum: u128 = 0;
    for x in &latencies {
        sum = sum + x;
    }
    let avg = sum as f32 / latencies.len() as f32;

    println!("client{} -> server{} :: {} with {}samples", args.idx, args.serveridx, format!("{}ms", avg).yellow(), latencies.len());
    // println!("Client{}: latency={:?}, count={:?}.", args.idx, average_latencies, count);
    Ok(())
}

fn tcp_client(term: Arc<AtomicBool>,
              latencies: &mut Vec<u128>,
              host_config: &HashMap<String, Vec<String>>
) -> Result<(), EnvTestError> {
    let args = Args::parse();
    let dir = &host_config["log-dir"][0];
    let log_file = format!("{}env_client{}_{}_rtt.log", dir, args.idx, args.serveridx);
    let mut log = std::fs::File::create(&log_file)
        .map_err(|_| EnvTestError::FileOpError)?;

    while !term.load(Ordering::Relaxed) {
        let idx = args.serveridx;
        let host = &host_config["server-hosts"][idx];
        let port = &host_config["server-ports"][idx];

        let sent = SystemTime::now();
        //Insert latency
        thread::sleep(time::Duration::from_millis(args.latency));
        
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(addr)
            .map_err(|_| EnvTestError::TcpConnError)?;

        let mut rx_bytes = [0u8; 64];
        stream.read(&mut rx_bytes)
            .map_err(|_| EnvTestError::TcpReadError)?;

        // Measure RTT
        let duration = sent.elapsed().unwrap();
        latencies.push(duration.as_millis());

        // Log RTT
        log.write_all(&format!("{:?}\n", duration).as_bytes())
            .map_err(|_| EnvTestError::FileOpError)?;

        thread::sleep(time::Duration::from_millis(50));
    }

    Ok(())
}
