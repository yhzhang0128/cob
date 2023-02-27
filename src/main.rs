pub mod cli;
pub mod error;
pub mod config;

use openssh::*;
use clap::Parser;
use cli::{Cli, Action};
use crate::config::{
    TargetType,
    read_host_config,
    read_latency_config,
};
use crate::error::OracleError;

#[tokio::main]
async fn main() -> Result<(), OracleError> {
    // Parse the target from arguments
    let cli = Cli::parse();
    let mut target = TargetType::Unknown;
    match cli.action {
        Action::Eval { target_arg } => {
            match target_arg.as_str() {
                "testenv" => { target = TargetType::TestEnv }
                "hotstuff" => { target = TargetType::HotStuff }
                "pompe" => { target = TargetType::Pompe }
                _ => { Err(OracleError::UnknownTarget)? }
            }
        }
    };
    println!("Target: {:?}", target);

    // Config network latency
    let host_config = read_host_config()?;
    println!("{:?}", host_config);
    
    let latency_matrix = read_latency_config()?;
    println!("{:?}", latency_matrix);

    // Example of openssh
    // let session = Session::connect("ssh://Yunhao@server0", KnownHosts::Accept)
    //     .await
    //     .map_err(|_| OracleError::SshConnFailed)?;

    // let whoami = session.command("whoami").output()
    //     .await
    //     .map_err(|_| OracleError::SshCommandFailed)?;
    // println!("Ssh succeeds: {}", String::from_utf8(whoami.stdout).unwrap());

    // session.close()
    //     .await
    //     .map_err(|_| OracleError::SshCloseFailed)?;


    Ok(())
}
