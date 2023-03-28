use openssh::*;
use indicatif::ProgressBar;
use std::collections::HashMap;
use crate::error::OracleError;

pub async fn start_ssh_conns(hosts: &Vec<String>) -> Result<HashMap<String, Session>, OracleError> {
    let mut result = HashMap::new();
    let bar = ProgressBar::new(hosts.len().try_into().unwrap());

    for host in hosts {
        let cmd = format!("ssh://{}@{}", "Yunhao", host);
        let session = Session::connect(cmd.as_str(), KnownHosts::Accept)
            .await
            .map_err(|_| OracleError::SshConnFailed)?;

        // remove any network delay
        // sudo tc qdisc del dev enp1s0d1 root netem
        session.command("sudo")
            .args(["tc", "qdisc", "del", "dev", "enp1s0d1", "root", "netem"])
            .output()
            .await
            .map_err(|_| OracleError::SshCommandFailed)?;

        bar.inc(1);
        result.insert(String::from(host), session);
    }

    bar.finish();
    return Ok(result);
}

pub async fn close_ssh_conns(sessions: HashMap<String, Session>) -> Result<(), OracleError> {
    for (_, s) in sessions {
        s.close()
            .await
            .map_err(|_| OracleError::SshCloseFailed)?;
    }
    Ok(())
}
