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
    Pompe,
    //Themis,
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
    #[clap(about = "Setup latency emulation.")]
    Latency {
        #[clap(
            short = 's', long = "setup",
            default_value = "geo",
            help = "Specify the setup for latency emulation (local or geo)."

        )]
        setup_str: String,
    },
    #[clap(about = "Kill processes on remote hosts.")]
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
            default_value = "30000",
            help = "Specify the duration of experiment in ms."
        )]
        duration: u64,
    },
    #[clap(about = "List target systems supported.")]
    List {},
}
