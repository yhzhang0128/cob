use openssh::*;
use indicatif::ProgressBar;
use crate::error::OracleError;

pub async fn start_ssh_conns(hosts: &Vec<String>) -> Result<Vec<Session>, OracleError> {
    let mut result = vec![];
    let num = hosts.len().try_into().unwrap();
    let bar = ProgressBar::new(num);

    println!("Start {} ssh connections", num);
    for host in hosts {
        let cmd = format!("ssh://{}@{}", "Yunhao", host);
        let session = Session::connect(cmd.as_str(), KnownHosts::Accept)
            .await
            .map_err(|_| OracleError::SshConnFailed)?;

        bar.inc(1);
        result.push(session);
    }

    bar.finish();
    return Ok(result);
}

pub async fn close_ssh_conns(sessions: Vec<Session>) -> Result<(), OracleError> {
    for s in sessions {
        s.close()
            .await
            .map_err(|_| OracleError::SshCloseFailed)?;
    }
    
    Ok(())
}
