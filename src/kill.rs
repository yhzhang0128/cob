use crate::error::OracleError;
use crate::ssh::start_ssh_conns;
use crate::config::read_host_config;

pub async fn killall() -> Result<(), OracleError>{
    // Start ssh connections
    let host_config = read_host_config()?;
    let ssh_conns = start_ssh_conns(&host_config["hostnames"]).await?;

    let client_bin = &host_config["binary-files"][0];
    let server_bin = &host_config["binary-files"][1];

    for (host, s) in ssh_conns {
        println!("Kill process {} on host {}.", client_bin, host);
        let kill1 = s.command("killall")
            .args([client_bin.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        if kill1.stderr.len() > 0 {
            println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(kill1.stderr).unwrap());
        }

        println!("Kill process {} on host {}.", server_bin, host);
        let kill2 = s.command("killall")
            .args([server_bin.as_str()])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;
        if kill2.stderr.len() > 0 {
            println!("  [warning] stderr from {}: {:?}", host, String::from_utf8(kill2.stderr).unwrap());
        }
    }

    Ok(())
}
