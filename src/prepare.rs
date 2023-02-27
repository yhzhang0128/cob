use openssh::*;
use std::process::Command;
use indicatif::ProgressBar;
use std::collections::HashMap;
use crate::error::OracleError;

pub async fn prepare_files(ssh_conns: &Vec<Session>, config: &HashMap<String, Vec<String>>) -> Result<(), OracleError> {
    let local_bin_dir = &config["local-dir"][0];
    let remote_bin_dir = &config["remote-dir"][0];
    let local_config_dir = &config["local-dir"][1];
    let remote_config_dir = &config["remote-dir"][1];

    // Create directories for copying the client/server binaries
    println!("[3/6] Creat binary and config directories on all the hosts.");
    for s in ssh_conns {
        // Make directory for executable binaries
        let _mkdir = s.command("mkdir")
            .args(["-p", remote_bin_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        // Make directory for config files
        let _mkdir = s.command("mkdir")
            .args(["-p", remote_config_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
    }

    // Copy client and server binaries to remote hosts
    println!("[4/6] Copy binaries and configs to the hosts.");

    let hosts = &config["hostnames"];
    let num = hosts.len().try_into().unwrap();
    let bar = ProgressBar::new(num);

    for host in hosts {
        // Copy executable binary files
        for bin in &config["binary-files"] {
            let file = format!("{}{}", local_bin_dir, bin);
            let bin_dir = format!("{}:{}", host, remote_bin_dir);
            Command::new("scp")
                .args([file.as_str(), bin_dir.as_str()])
                .output()
                .map_err(|_| OracleError::BinaryCopyFailed)?;
        }
        // Copy config files
        for con in &config["config-files"] {
            let file = format!("{}{}", local_config_dir, con);
            let config_dir = format!("{}:{}", host, remote_config_dir);
            Command::new("scp")
                .args([file.as_str(), config_dir.as_str()])
                .output()
                .map_err(|_| OracleError::BinaryCopyFailed)?;
        }
        
        bar.inc(1);
    }
    bar.finish();

    Ok(())
}
