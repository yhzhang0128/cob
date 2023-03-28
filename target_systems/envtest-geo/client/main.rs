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
    #[arg(long)]
    config: String,
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
        .add_source(File::with_name(args.config.as_str()))
        .build()
        .map_err(|_| EnvTestError::ConfigError)?;
    let config = config_builder
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .map_err(|_| EnvTestError::ConfigError)?;

    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGTERM kill signal
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))
        .map_err(|_| EnvTestError::SigTermHandlerError)?;

    // Execute TCP client logic
    let mut latencies: Vec<u128> = vec![];
    match tcp_client(term, &mut latencies, &config) {
        Ok(()) => {}
        Err(_err) => { /*TCP error can occur when experiment terminates*/ }
    }

    // Client terminated by signal, print latency info
    let mut sum: u128 = 0;
    for x in &latencies {
        sum = sum + x;
    }

    if latencies.len() == 0 {
        println!("client->server [{}->{}] = {:.1}ms <- {}ms, {} samples", args.idx, args.serveridx, 0, 0, args.latency);
    } else {
        let avg = sum as f32 / latencies.len() as f32;
        println!("client->server [{}->{}] = {:.1}ms <- {}ms, {} samples", args.idx, args.serveridx, avg, latencies.len(), args.latency);
    }

    Ok(())
}

fn tcp_client(term: Arc<AtomicBool>,
              latencies: &mut Vec<u128>,
              config: &HashMap<String, Vec<String>>
) -> Result<(), EnvTestError> {
    let args = Args::parse();
    let dir = &config["log-dir"][0];
    let log_file = format!("{}env_client{}_{}_rtt.log", dir, args.idx, args.serveridx);
    let mut log = std::fs::File::create(&log_file)
        .map_err(|_| EnvTestError::FileOpError)?;

    let host = &config["server-hosts"][args.serveridx];
    let port = &config["server-ports"][args.serveridx];
    //println!("Client{} listens to host {} port {}", args.idx, host, port);
    
    while !term.load(Ordering::Relaxed) {
        //Insert latency (obsolete, use tc instead)
        //thread::sleep(time::Duration::from_millis(args.latency));
        
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(addr)
            .map_err(|_| EnvTestError::TcpConnError)?;

        let sent = SystemTime::now();

        // Send message
        let mut rx_bytes = [0u8; 64];
        stream.write_all(&mut rx_bytes)
            .map_err(|_| EnvTestError::TcpWriteError)?;

        // Receive message
        stream.read(&mut rx_bytes)
            .map_err(|_| EnvTestError::TcpReadError)?;

        // Measure RTT
        let duration = sent.elapsed().unwrap().as_millis();
        if duration != 0 {  // Naive test that the server is alive
            latencies.push(duration);
        }

        // Log RTT
        log.write_all(&format!("{:?}\n", duration).as_bytes())
            .map_err(|_| EnvTestError::FileOpError)?;

        thread::sleep(time::Duration::from_millis(50));
    }

    Ok(())
}
