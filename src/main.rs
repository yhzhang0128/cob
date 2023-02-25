pub mod cli;
pub mod error;
pub mod config;

use openssh::*;
use clap::Parser;
use cli::{Cli, Action};
use crate::error::OracleError;

#[tokio::main]
async fn main() -> Result<(), OracleError> {
    // Parse arguments
    let cli = Cli::parse();
    match cli.action {
        Action::Eval { target } => {
            println!("target is {}", target);
        }
    }

    // Config network latency
    config::config_latency()?;

    // Example of openssh
    let session = Session::connect("ssh://Yunhao@server0", KnownHosts::Accept)
        .await
        .map_err(|_| OracleError::SshConnFailed)?;

    let whoami = session.command("whoami").output()
        .await
        .map_err(|_| OracleError::SshCommandFailed)?;
    println!("Ssh succeeds: {}", String::from_utf8(whoami.stdout).unwrap());

    session.close()
        .await
        .map_err(|_| OracleError::SshCloseFailed)?;


    Ok(())
}
