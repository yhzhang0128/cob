use {
    clap::{
        Parser,
        Subcommand,
    },
};

#[derive(Debug)]
pub enum TargetType {
    EnvTest,
    EnvTestGeo,
    HotStuff,
    HotStuffBumped,
    Pompe,
    PompeBumped,
    Unknown,
}

#[derive(Parser, Debug)]
#[clap(about = "A cli for the chance oracle benchmark")]
pub struct Cli {
    #[clap(subcommand)]
    pub action:     Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    #[clap(about = "Kill processes on remote hots.")]
    Kill {
        #[clap(
            short = 't', long = "target",
            default_value = "envtest",
            help = "Specify the target system to kill."
        )]
        target_str: String,
    },
    #[clap(about = "Evaluate a target system.")]
    Eval {
        #[clap(
            short = 't', long = "target",
            default_value = "envtest",
            help = "Specify the target system for evaluation."
        )]
        target_str: String,
        #[clap(
            short = 'd', long = "duration",
            default_value = "10000",
            help = "Specify the duration of experiment in ms."
        )]
        duration: u64,
    },
}
