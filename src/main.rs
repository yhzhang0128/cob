pub mod cli;
pub mod ssh;
pub mod error;
pub mod config;

use cli::parse_target_type;

use crate::config::{
    read_host_config,
    read_latency_config,
};
use crate::ssh::{
    start_ssh_conns,
    close_ssh_conns,
};

use std::process::Command;
use indicatif::ProgressBar;
use crate::error::OracleError;

#[tokio::main]
async fn main() -> Result<(), OracleError> {
    // Parse the target from arguments
    let target = parse_target_type()?;
    println!("Evaluation target: {:?}", target);

    // Start ssh connections
    let host_config = read_host_config()?;
    let ssh_conns = start_ssh_conns(&host_config["hostnames"]).await?;
    
    // Setup network latency emulation
    let latency_matrix = read_latency_config()?;
    println!("TODO: setup latency: {:?}", latency_matrix);

    // Create directories for copying the target binary
    for s in &ssh_conns {
        let _mkdir = s.command("mkdir")
            .args(["-p", "/opt/chance/target_binary"])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
    }
    println!("Created /opt/chance/target_binary on all the hosts.");

    // Copy client and server binary to remote hosts
    println!("Copy target binaries to the hosts.");
    let num = host_config["hostnames"].len().try_into().unwrap();
    let bar = ProgressBar::new(num);

    for host in &host_config["hostnames"] {
        let dir = format!("{}:/opt/chance/target_binary/", host);

        Command::new("scp")
            .args(["/opt/chance/cob/target/debug/envtest_client", dir.as_str()])
            .output()
            .map_err(|_| OracleError::BinaryCopyFailed)?;
        Command::new("scp")
            .args(["/opt/chance/cob/target/debug/envtest_server", dir.as_str()])
            .output()
            .map_err(|_| OracleError::BinaryCopyFailed)?;
        bar.inc(1);
    }
    bar.finish();

    // Execute servers and clients through the ssh connections
    for s in &ssh_conns {
        let client = s.command("/opt/chance/target_binary/envtest_client")
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        let server = s.command("/opt/chance/target_binary/envtest_server")
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        println!("Client: {}", String::from_utf8(client.stdout).unwrap());
        println!("Server: {}", String::from_utf8(server.stdout).unwrap());
    }

    // Stop experiments and collect results

    // Close ssh connections
    close_ssh_conns(ssh_conns).await?;

    Ok(())
}
