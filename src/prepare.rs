use openssh::*;
use std::process::Command;
use indicatif::ProgressBar;
use std::collections::HashMap;
use crate::error::OracleError;

pub async fn prepare_files(ssh_conns: &HashMap<String, Session>, config: &HashMap<String, Vec<String>>) -> Result<(), OracleError> {
    let local_bin_dir = &config["local-dir"][0];
    let remote_bin_dir = &config["remote-dir"][0];
    let local_config_dir = &config["local-dir"][1];
    let remote_config_dir = &config["remote-dir"][1];
    let log_dir = format!("{}*", &config["log-dir"][0]);

    // Create directories for copying the client/server binaries
    println!("[3/7] Setup directories for log, binary and config files on remote hosts.");
    for (_, s) in ssh_conns {
        // Cleanup directory for logs
        s.command("rm")
            .args(["-rf", log_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;

        // Make directory for executable binaries
        s.command("mkdir")
            .args(["-p", remote_bin_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        // Make directory for config files
        s.command("mkdir")
            .args(["-p", remote_config_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
    }

    // Copy client and server binaries to remote hosts
    println!("[4/7] Copy binary and config files to remote hosts.");

    let hosts = &config["hostnames"];
    let num = hosts.len().try_into().unwrap();
    let bar = ProgressBar::new(num);

    for host in hosts {
        // Copy executable binary files
        for bin in &config["binary-files"] {
            let file = format!("{}{}", local_bin_dir, bin);
            let bin_dir = format!("{}:{}", host, remote_bin_dir);
            println!("scp {} {}", file.as_str(), bin_dir.as_str());
            Command::new("scp")
                .args([file.as_str(), bin_dir.as_str()])
                .output()
                .map_err(|_| OracleError::BinaryCopyFailed)?;
        }
        // Copy config files
        for con in &config["config-files"] {
            let file = format!("{}{}", local_config_dir, con);
            let config_dir = format!("{}:{}", host, remote_config_dir);
            println!("scp {} {}", file.as_str(), config_dir.as_str());
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
