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
    close_ssh_conns,
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
                "envtest" => { target = TargetType::EnvTest }
                "hotstuff" => { target = TargetType::HotStuff }
                "pompe" => { target = TargetType::Pompe }
                _ => { Err(OracleError::UnknownTarget)? }
            }
        }
    };
    println!("Evaluation target: {:?}", target);

    // Start ssh connections
    let host_config = read_host_config()?;
    let ssh_conns = start_ssh_conns(&host_config["hostnames"]).await?;
    
    // Setup network latency emulation
    let latency_matrix = read_latency_config()?;
    println!("TODO: latency matrix: {:?}", latency_matrix);

    // Create directories for copying the target binary
    for s in &ssh_conns {
        let _mkdir = s.command("mkdir -p /opt/chance/target_binary")
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        let ls = s.command("ls /opt/chance")
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        println!("Ssh ls: {}", String::from_utf8(ls.stdout).unwrap());
    }
    println!("Created /opt/chance/target_binary on all hosts.");

    // Copy client and server binary to remote hosts

    // Run servers and clients through the ssh connections

    // Stop experiments and collect results

    // Close ssh connections
    close_ssh_conns(ssh_conns).await?;

    Ok(())
}
