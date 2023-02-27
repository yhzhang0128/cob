pub mod cli;
pub mod ssh;
pub mod error;
pub mod config;
pub mod prepare;

use cli::parse_target_type;
use crate::error::OracleError;

use crate::ssh::{
    start_ssh_conns,
    close_ssh_conns,
};
use crate::config::{
    read_host_config,
    read_latency_config,
};
use crate::prepare::prepare_files;

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

    // Prepare the directories and binary files
    prepare_files(&ssh_conns, &host_config["hostnames"]).await?;

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
