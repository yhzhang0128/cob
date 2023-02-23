pub mod error;
pub mod config;

use openssh::*;
use crate::error::OracleError;


#[tokio::main]
async fn main() -> Result<(), OracleError> {
    config::config_latency()?;

    // Example of openssh
    let session = Session::connect("ssh://Yunhao@server0", KnownHosts::Accept)
        .await
        .map_err(|_| OracleError::SshFailed)?;

    let whoami = session.command("whoami").output().await.unwrap();
    println!("Ssh succeeds: {}", String::from_utf8(whoami.stdout).unwrap());

    session.close().await.unwrap();
    Ok(())
}
