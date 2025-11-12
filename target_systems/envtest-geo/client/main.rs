pub mod error;
use clap::Parser;
use config::{Config, File};
use std::collections::HashMap;
use crate::error::EnvTestError;

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
    idx: u32,
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
    let mut latencies: Vec<Vec<u128>> = vec![];
    for _i in 0..config["server-hosts"].len() {
        latencies.push(vec![]);
    }
    match tcp_client(term, &mut latencies, &config) {
        Ok(()) => {}
        Err(_err) => { /*TCP error can occur when experiment terminates*/ }
    }

    // Client terminated by signal, print latency info
    for serveridx in 0..config["server-hosts"].len() {
        let mut sum: u128 = 0;
        for x in &latencies[serveridx] {
            sum = sum + x;
        }

        if latencies[serveridx].len() == 0 {
            println!("client->server [{}->{}] = {:.1}ms", args.idx, serveridx, 0);
        } else {
            let avg = sum as f32 / latencies[serveridx].len() as f32;
            println!("client->server [{}->{}] = {:.1}ms, {} samples", args.idx, serveridx, avg, latencies[serveridx].len());
        }
    }

    Ok(())
}

fn tcp_client(term: Arc<AtomicBool>,
              latencies: &mut Vec<Vec<u128>>,
              config: &HashMap<String, Vec<String>>
) -> Result<(), EnvTestError> {
    let args = Args::parse();
    let dir = &config["log-dir"][0];
    let log_file = format!("{}env_client{}_rtt.log", dir, args.idx);
    let mut log = std::fs::File::create(&log_file)
        .map_err(|_| EnvTestError::FileOpError)?;



    let mut serveridx = 0;
    while !term.load(Ordering::Relaxed) {
        let host = &config["server-hosts"][serveridx];
        let port = &config["server-ports"][serveridx];

        //Insert latency (obsolete, use tc instead)
        //thread::sleep(time::Duration::from_millis(args.latency));
        
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(addr)
            .expect("TCP connect error!");

        let sent = SystemTime::now();

        // Send message
        let mut rx_bytes = [0u8; 64];
        stream.write_all(&mut rx_bytes)
            .expect("TCP write error!");

        // Receive message
        stream.read(&mut rx_bytes)
            .expect("TCP read error!");

        // Measure RTT
        let duration = sent.elapsed().unwrap().as_millis();
        if duration != 0 {  // Naive test that the server is alive
            latencies[serveridx].push(duration);
        }

        // Log RTT
        log.write_all(&format!("{:?}\n", duration).as_bytes())
            .map_err(|_| EnvTestError::FileOpError)?;

        serveridx = (serveridx + 1) % config["server-hosts"].len();
    }

    Ok(())
}
