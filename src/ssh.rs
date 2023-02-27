use openssh::*;
use crate::error::OracleError;

pub async fn start_ssh_conns(hosts: &Vec<String>) -> Result<Vec<Session>, OracleError> {
    let mut result = vec![];

    for host in hosts {
        let cmd = format!("ssh://{}@{}", "Yunhao", host);
        println!("{}", cmd);
        let session = Session::connect(cmd.as_str(), KnownHosts::Accept)
            .await
            .map_err(|_| OracleError::SshConnFailed)?;

        result.push(session);
    }

    return Ok(result);
}

pub async fn close_ssh_conns(_sessions: &Vec<Session>) -> Result<(), OracleError> {
    // TODO: how to close connections without compiler error
    
    // for s in sessions {
    //     s.close()
    //         .await
    //         .map_err(|_| OracleError::SshCloseFailed)?;
    // }
    
    Ok(())
}
