use {
    clap::{
        Parser,
        Subcommand,
    },
};

#[derive(Parser, Debug)]
#[clap(about = "A cli for the chance oracle benchmark")]
pub struct Cli {
    #[clap(subcommand)]
    pub action:     Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    #[clap(about = "Evaluate a target system.")]
    Eval {
        #[clap(
            short = 't', long = "target",
            default_value = "envtest",
            help = "Specify the target system for evaluation."
        )]
        target_arg: String,
    },
    #[clap(about = "Kill processes on remote hots.")]
    Kill {
    },
}

#[derive(Debug)]
pub enum TargetType {
    EnvTest,
    HotStuff,
    Pompe,
    Unknown,
}
