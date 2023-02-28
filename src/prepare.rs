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
    let remote_log_dir = &config["log-dir"][0];

    // Create directories for copying the client/server binaries
    println!("[3/7] Setup directories for log, binary and config files on remote hosts.");
    for (host, s) in ssh_conns {
        // Make directory for executable binaries
        let mkdir1 = s.command("mkdir")
            .args(["-p", remote_bin_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        if mkdir1.stderr.len() > 0 {
            println!("mkdir stderr on {}: {:?}", host, String::from_utf8(mkdir1.stderr).unwrap());
            Err(OracleError::SshCommandFailed)?
        }

        // Make directory for config files
        let mkdir2 = s.command("mkdir")
            .args(["-p", remote_config_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        if mkdir2.stderr.len() > 0 {
            println!("mkdir stderr on {}: {:?}", host, String::from_utf8(mkdir2.stderr).unwrap());
            Err(OracleError::SshCommandFailed)?
        }

        // Cleanup directory for logs
        let rm = s.command("rm")
            .args(["-rf", remote_log_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        if rm.stderr.len() > 0 {
            println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(rm.stderr).unwrap());
        }

        let mkdir3 = s.command("mkdir")
            .args(["-p", remote_log_dir.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        if mkdir3.stderr.len() > 0 {
            println!("mkdir stderr on {}: {:?}", host, String::from_utf8(mkdir3.stderr).unwrap());
            Err(OracleError::SshCommandFailed)?
        }


    }

    // Copy client and server binaries to remote hosts
    println!("[4/7] Copy binary and config files to remote hosts.");

    let hosts = &config["hostnames"];
    let file_per_host = config["binary-files"].len() * config["config-files"].len();
    let bar = ProgressBar::new((hosts.len() * file_per_host).try_into().unwrap());

    for host in hosts {
        // Copy executable binary files
        for bin in &config["binary-files"] {
            let file = format!("{}{}", local_bin_dir, bin);
            let bin_dir = format!("{}:{}", host, remote_bin_dir);
            Command::new("scp")
                .args([file.as_str(), bin_dir.as_str()])
                .output()
                .map_err(|_| OracleError::BinaryCopyFailed)?;
            bar.inc(1);
        }

        // Copy config files
        for con in &config["config-files"] {
            let file = format!("{}{}", local_config_dir, con);
            let config_dir = format!("{}:{}", host, remote_config_dir);
            Command::new("scp")
                .args([file.as_str(), config_dir.as_str()])
                .output()
                .map_err(|_| OracleError::BinaryCopyFailed)?;
            bar.inc(1);
        }
    }
    bar.finish();

    Ok(())
}
