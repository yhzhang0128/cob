use {
    clap::{
        Parser,
        Subcommand,
    },
};
use crate::error::OracleError;

#[derive(Parser, Debug)]
#[clap(about = "A cli for the chance oracle benchmark")]
pub struct Cli {
    #[clap(subcommand)]
    pub action:     Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    #[clap(about = "Evaluate a target system")]
    Eval {
        #[clap(
            short = 't', long = "target",
            default_value = "envtest",
            help = "Specify the target system"
        )]
        target_arg: String,
    },
}

#[derive(Debug)]
pub enum TargetType {
    EnvTest,
    HotStuff,
    Pompe,
    Unknown,
}

pub fn parse_target_type() -> Result<TargetType, OracleError> {
    let cli = Cli::parse();
    match cli.action {
        Action::Eval { target_arg } => {
            match target_arg.as_str() {
                "envtest" => { return Ok(TargetType::EnvTest) }
                "hotstuff" => { return Ok(TargetType::HotStuff) }
                "pompe" => { return Ok(TargetType::Pompe) }
                _ => { Err(OracleError::UnknownTarget)? }
            }
        }
    };
    Err(OracleError::UnknownTarget)?
}
