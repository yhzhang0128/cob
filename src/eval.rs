use std::{thread, time};
use indicatif::ProgressBar;

use crate::cli::TargetType;
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

const EXP_DURATION_MS: u64 = 5000;

pub async fn evaluate(target: TargetType) -> Result<(), OracleError>{
    println!("Evaluation target: {:?}", target);

    // Start ssh connections
    let host_config = read_host_config()?;
    let num_hosts = host_config["hostnames"].len();
    println!("[1/7] Start ssh connections to {} remote hosts.", num_hosts);
    let ssh_conns = start_ssh_conns(&host_config["hostnames"]).await?;
    
    // Setup network latency emulation
    let latency_matrix = read_latency_config()?;
    println!("[2/7] TODO: setup latency.");
    println!("  {:?}", latency_matrix);

    // Prepare the directories and binary files
    prepare_files(&ssh_conns, &host_config).await?;

    // Execute servers and clients through the ssh connections
    let mut clients = vec![];
    let mut servers = vec![];

    let binary_dir = &host_config["remote-dir"][0];
    let client_bin = &host_config["binary-files"][0];
    let server_bin = &host_config["binary-files"][1];
    let client_cmd = format!("{}{}", binary_dir, client_bin);
    let server_cmd = format!("{}{}", binary_dir, server_bin);

    let mut server_id = 0;
    for server in &host_config["server-hosts"] {
        match ssh_conns.get(server) {
            None => { Err(OracleError::InvalidServerHost)? }
            Some(s) => {
                servers.push(s.command(server_cmd.as_str())
                             .args(&host_config["server-args"])
                             .arg("--idx")
                             .arg(server_id.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        server_id += 1;
    }
    println!("[5/7] Execute {} servers on remote hosts.", server_id);
    thread::sleep(time::Duration::from_millis(1000));

    let mut client_id = 0;
    for client in &host_config["client-hosts"] {
        match ssh_conns.get(client) {
            None => { Err(OracleError::InvalidClientHost)? }
            Some(s) => {
                clients.push(s.command(client_cmd.as_str())
                             .args(&host_config["client-args"])
                             .arg("--idx")
                             .arg(client_id.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        client_id += 1;
    }
    println!("[6/7] Execute {} clients on remote hosts.", client_id);

    // Wait a duration and terminate the experiment
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(time::Duration::from_millis(120));
    let msg = format!("Executing remote client/server for {}ms.", EXP_DURATION_MS);
    pb.set_message(msg);
    thread::sleep(time::Duration::from_millis(EXP_DURATION_MS));
    let finish_msg = format!("Terminate experiment after {}ms.", EXP_DURATION_MS);
    pb.finish_with_message(finish_msg);

    // Collect output and close connections
    println!("[7/7] Close the ssh connections.");
    close_ssh_conns(ssh_conns).await?;

    Ok(())
}
