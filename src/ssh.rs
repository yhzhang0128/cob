use openssh::*;
use indicatif::ProgressBar;
use std::collections::HashMap;
use crate::error::OracleError;

pub async fn start_ssh_conns(hosts: &Vec<String>) -> Result<HashMap<String, Session>, OracleError> {
    let mut result = HashMap::new();
    let num = hosts.len().try_into().unwrap();
    let bar = ProgressBar::new(num);

    println!("[1/6] Start ssh connections to {} hosts.", num);
    for host in hosts {
        let cmd = format!("ssh://{}@{}", "Yunhao", host);
        let session = Session::connect(cmd.as_str(), KnownHosts::Accept)
            .await
            .map_err(|_| OracleError::SshConnFailed)?;

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
