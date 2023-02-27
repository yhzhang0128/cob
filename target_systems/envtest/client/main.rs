use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("/users/Yunhao/client_log.txt")?;
    file.write_all(b"This is envtest client.\n")?;

    println!("This is envtest client.");
    Ok(())
}
