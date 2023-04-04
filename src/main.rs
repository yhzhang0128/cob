pub mod cli;
pub mod ssh;
pub mod eval;
pub mod kill;
pub mod prep;
pub mod error;
pub mod spawn;
pub mod config;

use cli::*;
use clap::Parser;
use kill::killall;
use eval::evaluate;
use crate::error::OracleError;

#[tokio::main]
async fn main() -> Result<(), OracleError> {
    let cli = Cli::parse();

    match cli.action {
        Action::Kill { target_str } => {
            let target = target_type(&target_str)?;
            killall(&target, true).await?
        }

        Action::Eval { target_str, duration } => {
            let target = target_type(&target_str)?;
            match evaluate(&target, duration).await {
                Ok(()) => { killall(&target, false).await?; }
                Err(err) => {
                    killall(&target, false).await.unwrap();
                    Err(err)?
                }
            }
        }
    };

    Ok(())
}

fn target_type(target_str: &String) -> Result<TargetType, OracleError> {
    match target_str.as_str() {
        "envtest" => { return Ok(TargetType::EnvTest); }
        "envtest-geo" => { return Ok(TargetType::EnvTestGeo); }
        "hotstuff" => { return Ok(TargetType::HotStuff); }
        "hotstuff-bump" => { return Ok(TargetType::HotStuffBumped); }
        "pompe" => { return Ok(TargetType::Pompe); }
        _ => { Err(OracleError::UnknownTarget)? }
    }

}
