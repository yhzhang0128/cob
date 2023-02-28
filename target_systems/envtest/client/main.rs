use clap::Parser;
// use std::fs::File;
// use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   config: String,

   #[arg(short, long)]
   log: String,

   #[arg(short, long)]
   idx: u8,
}


fn main() -> std::io::Result<()> {
    // let mut file = File::create("/users/Yunhao/client_log.txt")?;
    // file.write_all(b"This is envtest client.\n")?;
    let args = Args::parse();

    println!("This is envtest client#{}.", args.idx);
    loop {};
}
