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
    let mut latency_history = HashMap::<&String, Vec<u128>>::new();
    for host in &host_config["server-hosts"] {
        latency_history.insert(host, vec![]);
    }

    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGTERM kill signal
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))
        .map_err(|_| EnvTestError::SigTermHandlerError)?;

    // Execute TCP client logic
    match tcp_client(term, &host_config, &mut latency_history) {
        Ok(()) => {}
        Err(_err) => { /*TCP error can occur when experiment terminates*/ }
    }

    // Client terminated by signal, print latency info
    let mut count = vec![];
    let mut average_latencies = vec![];
    for host in &host_config["server-hosts"] {
        let mut sum: u128 = 0;
        for x in &latency_history[host] {
            sum = sum + x;
        }

        let avg = sum as f32 / latency_history[host].len() as f32;
        average_latencies.push(avg);
        count.push(latency_history[host].len());
    }
    
    println!("Client{}: latency={:?}, count={:?}.", args.idx, average_latencies, count);
    Ok(())
}

fn tcp_client(term: Arc<AtomicBool>,
              host_config: &HashMap<String, Vec<String>>,
              latency_history: &mut HashMap<&String, Vec<u128>>
) -> Result<(), EnvTestError> {
    let args = Args::parse();
    let dir = &host_config["log-dir"][0];
    let log_file = format!("{}env_client{}_rtt.log", dir, args.idx);
    let mut log = std::fs::File::create(&log_file)
        .map_err(|_| EnvTestError::FileOpError)?;
    println!("This is envtest client#{} logging to {}.", args.idx, log_file);

    while !term.load(Ordering::Relaxed) {
        let num_servers = host_config["server-hosts"].len();
        for idx in 0..num_servers {
            let host = &host_config["server-hosts"][idx];
            let port = &host_config["server-ports"][idx];

            let sent = SystemTime::now();
            let addr = format!("{}:{}", host, port);
            let mut stream = TcpStream::connect(addr)
                .map_err(|_| EnvTestError::TcpConnError)?;

            let mut rx_bytes = [0u8; 64];
            stream.read(&mut rx_bytes)
                .map_err(|_| EnvTestError::TcpReadError)?;

            let duration = sent.elapsed().unwrap();
            latency_history.get_mut(host).map(|val| val.push(duration.as_millis()));
            log.write_all(&format!("{:?}\n", duration).as_bytes())
                .map_err(|_| EnvTestError::FileOpError)?;

            thread::sleep(time::Duration::from_millis(50));
        }
    }

    Ok(())
}


#[tokio::main]
async fn _main() -> Result<(), EnvTestError> {

    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGTERM kill signal
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))
        .map_err(|_| EnvTestError::SigTermHandlerError)?;

    println!("here1");

    while !term.load(Ordering::Relaxed) {
        thread::sleep(time::Duration::from_millis(100));
    }

    println!("here2");
    let mut latency = std::fs::File::create("/home/yunhao/tmp.txt")
        .map_err(|_| EnvTestError::FileOpError)?;

    println!("here3");
    let row1 = format!("Write to file after terminated\n");
    latency.write_all(&row1.as_bytes())
        .map_err(|_| EnvTestError::FileOpError)?;
    println!("here4");

    Ok(())
}
