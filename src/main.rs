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
    prepare_files(&ssh_conns,
                  &host_config["hostnames"],
                  &host_config["binaries"]).await?;

    // Execute servers and clients through the ssh connections
    let duration = 10000;
    println!();
    for s in &ssh_conns {
        let binaries = &host_config["binaries"];
        let client_cmd = format!("{}{}", binaries[4], binaries[1]);
        let server_cmd = format!("{}{}", binaries[4], binaries[3]);

        // TODO: if client/server failed, this may not return error
        let _client = s.command(client_cmd.as_str());
        let _server = s.command(server_cmd.as_str());
    }

    // Wait a duration and terminate the experiment
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(time::Duration::from_millis(120));
    let spin_msg = format!("Executing remote client/server binaries for {}ms.", duration);
    pb.set_message(spin_msg);
    thread::sleep(time::Duration::from_millis(duration));

    close_ssh_conns(ssh_conns).await?;
    let finish_msg = format!("Finish experiment after {}ms.", duration);
    pb.finish_with_message(finish_msg);

    // Collect experimental results
    // TODO

    Ok(())
}
