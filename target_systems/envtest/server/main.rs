use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("/users/Yunhao/client_log.txt")?;
    file.write_all(b"This is envtest server.")?;

    println!("This is envtest server.");
    Ok(())
}

