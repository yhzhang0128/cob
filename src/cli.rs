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
    #[clap(about = "Evaluate a target system")]
    Eval {
        #[clap(
            short = 's', long = "sys",
            default_value = "testenv",
            help = "Specify the target system"
        )]
        target_arg: String,
    },
}

