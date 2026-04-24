pub mod cli;
pub mod ssh;
pub mod eval;
pub mod kill;
pub mod prep;
pub mod error;
pub mod spawn;
pub mod config;
pub mod latency;

use cli::*;
use clap::Parser;
use kill::killall;
use eval::evaluate;
use latency::setup_latency;
use latency::remove_latency;
use crate::error::OracleError;

#[tokio::main]
async fn main() -> Result<(), OracleError> {
    let cli = Cli::parse();

    match cli.action {
        Action::Latency { setup_str } => {
            match setup_str.as_str() {
                "geo" => { setup_latency().await?; }
                "local" => { remove_latency().await?; }
                _ => { println!("Usage: cargo run latency -s [geo | local]") }
            }
        }

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
        Action::List { } => {
            println!("hotstuff");
            println!("pompe");
            println!("envtest");
            println!("envtest-geo");
        }
    };

    Ok(())
}

fn target_type(target_str: &String) -> Result<TargetType, OracleError> {
    match target_str.as_str() {
        "hotstuff" => { return Ok(TargetType::HotStuff); }
        "pompe" => { return Ok(TargetType::Pompe); }
        "envtest" => { return Ok(TargetType::EnvTest); }
        "envtest-geo" => { return Ok(TargetType::EnvTestGeo); }
        _ => { Err(OracleError::UnknownTarget)? }
    }

}
