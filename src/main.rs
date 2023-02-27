pub mod cli;
pub mod ssh;
pub mod error;
pub mod config;
pub mod prepare;

use std::{thread, time};
use cli::parse_target_type;
use indicatif::ProgressBar;
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
    prepare_files(&ssh_conns, &host_config).await?;

    // Execute servers and clients through the ssh connections
    let duration = 10000;
    println!();
    for s in &ssh_conns {
        let binary_dir = &host_config["remote-dir"][0];
        let _config_dir = &host_config["remote-dir"][1];

        let client_bin = &host_config["binary-files"][0];
        let server_bin = &host_config["binary-files"][1];

        let client_cmd = format!("{}{}", binary_dir, client_bin);
        let server_cmd = format!("{}{}", binary_dir, server_bin);

        // TODO: if client/server failed, this may not return error
        println!("execute: {}", client_cmd);
        let _client = s.command(client_cmd.as_str())
            .args(&host_config["client-args"])
            .spawn()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;

        println!("execute: {}", server_cmd);
        let _server = s.command(server_cmd.as_str())
            .args(&host_config["server-args"])
            .spawn()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
    }

    // Wait a duration and terminate the experiment
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(time::Duration::from_millis(120));
    let msg = format!("Executing remote client/server for {}ms.", duration);
    pb.set_message(msg);
    thread::sleep(time::Duration::from_millis(duration));

    let finish_msg = format!("Finish experiment after {}ms.", duration);
    pb.finish_with_message(finish_msg);
    close_ssh_conns(ssh_conns).await?;

    // Collect experimental results
    // TODO

    Ok(())
}
