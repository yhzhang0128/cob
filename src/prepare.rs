use openssh::*;
use std::process::Command;
use indicatif::ProgressBar;
use crate::error::OracleError;

pub async fn prepare_files(ssh_conns: &Vec<Session>, hosts: &Vec<String>) -> Result<(), OracleError> {
    // Create directories for copying the client/server binaries
    for s in ssh_conns {
        let _mkdir = s.command("mkdir")
            .args(["-p", "/opt/chance/target_binary"])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
    }
    println!("Created /opt/chance/target_binary on all the hosts.");

    // Copy client and server binaries to remote hosts
    println!("Copy client/server binaries to the hosts.");
    let num = hosts.len().try_into().unwrap();
    let bar = ProgressBar::new(num);

    for host in hosts {
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

    Ok(())
}
