use openssh::*;
use std::process::Command;
use indicatif::ProgressBar;
use crate::error::OracleError;

pub async fn prepare_files(ssh_conns: &Vec<Session>, hosts: &Vec<String>, binaries: &Vec<String>) -> Result<(), OracleError> {
    // Create directories for copying the client/server binaries
    println!("Creat {} on all the hosts.", binaries[2]);
    for s in ssh_conns {
        let _mkdir = s.command("mkdir")
            .args(["-p", binaries[2].as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
    }

    // Copy client and server binaries to remote hosts
    println!("Copy client/server binaries to the hosts.");
    let num = hosts.len().try_into().unwrap();
    let bar = ProgressBar::new(num);

    for host in hosts {
        let dir = format!("{}:{}", host, binaries[2].as_str());

        Command::new("scp")
            .args([binaries[0].as_str(), dir.as_str()])
            .output()
            .map_err(|_| OracleError::BinaryCopyFailed)?;
        Command::new("scp")
            .args([binaries[1].as_str(), dir.as_str()])
            .output()
            .map_err(|_| OracleError::BinaryCopyFailed)?;
        bar.inc(1);
    }
    bar.finish();

    Ok(())
}
