use openssh::*;
use config::{Config, File};
use std::collections::HashMap;


#[derive(Debug)]
pub enum OracleError {
    ConfigError
}

#[tokio::main]
async fn main() -> Result<(), OracleError> {
    // Example of openssh
    let session = Session::connect("ssh://Yunhao@server0", KnownHosts::Accept)
        .await
        .unwrap();

    let ls = session.command("ls").output().await.unwrap();
    println!("{}", String::from_utf8(ls.stdout).unwrap());

    let whoami = session.command("whoami").output().await.unwrap();
    println!("{}", String::from_utf8(whoami.stdout).unwrap());

    session.close().await.unwrap();

    // Reading machine configuration
    let machine_config = Config::builder()
        .add_source(File::with_name("config/machine"))
        .build().unwrap();  //unwrap_or(OracleError::ConfigError)?;
    let machines = machine_config
            .try_deserialize::<HashMap<String, Vec<String>>>()
        .unwrap();
    println!("# Machine configuration:");
    println!("{:?}", machines);

    // Reading latency configuration
    let latency_config = Config::builder()
        .add_source(File::with_name("config/latency"))
        .build().unwrap();  //unwrap_or(OracleError::ConfigError)?;
    let latencies = latency_config
            .try_deserialize::<HashMap<String, Vec<String>>>()
            .unwrap();

    let mut latency_matrix : HashMap<String, Vec<i32>> = HashMap::new();
    for location in &latencies["locations"] {
        let mut tmp : Vec<i32> = Vec::new();
        for latency in &latencies[location] {
            tmp.push(latency.parse::<i32>().unwrap());
        }
        latency_matrix.insert(location.to_string(), tmp);
    }
    println!("{:?}", latency_matrix);    
    Ok(())
}
