pub mod cli;
pub mod ssh;
pub mod eval;
pub mod error;
pub mod config;
pub mod prepare;

use cli::*;
use clap::Parser;
use eval::evaluate;
use crate::error::OracleError;

#[tokio::main]
async fn main() -> Result<(), OracleError> {
    let cli = Cli::parse();

    match cli.action {
        Action::Eval { target_arg } => {
            match target_arg.as_str() {
                "envtest" => { evaluate(TargetType::EnvTest).await?; }
                "hotstuff" => { evaluate(TargetType::HotStuff).await?; }
                "pompe" => { evaluate(TargetType::Pompe).await?; }
                _ => { Err(OracleError::UnknownTarget)? }
            }
        }
    };

    Ok(())
}
