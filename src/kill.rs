use std::time;
use std::thread;
use crate::TargetType;
use crate::error::OracleError;
use crate::ssh::start_ssh_conns;
use crate::config::read_config;

pub async fn killall(target: &TargetType, print: bool) -> Result<(), OracleError>{
    // Start ssh connections
    let config = read_config(target)?;
    let ssh_conns = start_ssh_conns(&config["hostnames"]).await?;

    let client_bin = &config["binary-files"][0];
    let server_bin = &config["binary-files"][1];

    // Kill client processes
    for (host, s) in &ssh_conns {
        println!("Killing client on host {}.", host);
        let kill1 = s.command("killall")
            .args([client_bin.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        if print {
            println!("Kill process {} on host {}.", client_bin, host);
            if kill1.stderr.len() > 0 {
                println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(kill1.stderr).unwrap());
            }
        }
        thread::sleep(time::Duration::from_millis(10));
    }
    thread::sleep(time::Duration::from_millis(1000));

    // Kill speedbump processes
    if config["binary-files"].len() == 4 {
        let speedbump_bin = &config["binary-files"][2];
        for (host, s) in &ssh_conns {
            let kill1 = s.command("killall")
                .args([speedbump_bin.as_str()])
                .output()
                .await
                .map_err(|_| OracleError::SshCommandFailed)?;
            if print {
                println!("Kill process {} on host {}.", speedbump_bin, host);
                if kill1.stderr.len() > 0 {
                    println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(kill1.stderr).unwrap());
                }
            }
        }
    }
    
    // Kill server processes
    for (host, s) in &ssh_conns {
        let kill3 = s.command("killall")
            .args([server_bin.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        if print {
            println!("Kill process {} on host {}.", server_bin, host);
            if kill3.stderr.len() > 0 {
                println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(kill3.stderr).unwrap());
            }
        }
    }

    Ok(())
}
