use crate::TargetType;
use crate::error::OracleError;
use crate::ssh::start_ssh_conns;
use crate::config::read_config;

pub async fn killall(target: &TargetType, print: bool) -> Result<(), OracleError>{
    // Start ssh connections
    let config = read_config(target)?;
    let ssh_conns = start_ssh_conns(&config["hostnames"]).await?;

    // Kill all processes
    for bin in &config["binary-files"] {
        //println!("Killing {}", bin);
        for (host, s) in &ssh_conns {
            let kill = s.command("killall")
                .args([bin.as_str()])
                .output()
                .await
                .map_err(|_| OracleError::SshCommandFailed)?;
            if print {
                println!("Kill process {} on host {}.", bin, host);
                if kill.stderr.len() > 0 {
                    println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(kill.stderr).unwrap());
                }
            }
        }
    }

    // let client_bin = &config["binary-files"][0];
    // let server_bin = &config["binary-files"][1];
    // Kill client processes
    // for (host, s) in &ssh_conns {
    //     let kill1 = s.command("killall")
    //         .args([client_bin.as_str()])
    //         .output()
    //         .await
    //         .map_err(|_| OracleError::SshCommandFailed)?;
    //     if print {
    //         println!("Kill process {} on host {}.", client_bin, host);
    //         if kill1.stderr.len() > 0 {
    //             println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(kill1.stderr).unwrap());
    //         }
    //     }
    // }

    // Kill server processes
    // for (host, s) in &ssh_conns {
    //     let kill2 = s.command("killall")
    //         .args([server_bin.as_str()])
    //         .output()
    //         .await
    //         .map_err(|_| OracleError::SshCommandFailed)?;
    //     if print {
    //         println!("Kill process {} on host {}.", server_bin, host);
    //         if kill2.stderr.len() > 0 {
    //             println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(kill2.stderr).unwrap());
    //         }
    //     }
    // }

    Ok(())
}
