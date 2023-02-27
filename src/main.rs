pub mod cli;
pub mod ssh;
pub mod error;
pub mod config;

use clap::Parser;
use cli::{
    Cli,
    Action
};

use crate::config::{
    TargetType,
    read_host_config,
    read_latency_config,
};
use crate::ssh::{
    start_ssh_conns,
//    close_ssh_conns,
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

    // Test ssh connections
    let host_config = read_host_config()?;
    let ssh_conns = start_ssh_conns(&host_config["hostnames"]).await?;

    for s in &ssh_conns {
        let whoami = s.command("whoami").output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        println!("Ssh who: {}", String::from_utf8(whoami.stdout).unwrap());
    }

    
    //close_ssh_conns(&ssh_conns).await?;
    println!("{:?}", host_config["hostnames"]);
    
    // Config network latency
    let latency_matrix = read_latency_config()?;
    println!("{:?}", latency_matrix);

    Ok(())
}
