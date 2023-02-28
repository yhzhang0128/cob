pub mod cli;
pub mod ssh;
pub mod eval;
pub mod kill;
pub mod error;
pub mod config;
pub mod prepare;

use cli::*;
use clap::Parser;
use kill::killall;
use eval::evaluate;
use crate::error::OracleError;

#[tokio::main]
async fn main() -> Result<(), OracleError> {
    let cli = Cli::parse();

    match cli.action {
        Action::Kill { } => {
            killall(true).await?
        }
        Action::Eval { target_arg, duration } => {
            let mut target = TargetType::Unknown;
            match target_arg.as_str() {
                "envtest" => { target = TargetType::EnvTest; }
                "hotstuff" => { target = TargetType::HotStuff; }
                "pompe" => { target = TargetType::Pompe; }
                _ => { Err(OracleError::UnknownTarget)? }
            }

            match evaluate(target, duration).await {
                Ok(()) => { killall(false).await?; }
                Err(err) => {
                    killall(false).await.unwrap();
                    Err(err)?
                }
            }
        }
    };

    Ok(())
}
