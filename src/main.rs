use openssh::*;

#[tokio::main]
async fn main() {
    let session = Session::connect("ssh://Yunhao@server0", KnownHosts::Strict)
        .await
        .unwrap();

    let ls = session.command("ls").output().await.unwrap();
    println!("{}", String::from_utf8(ls.stdout).unwrap());

    let whoami = session.command("whoami").output().await.unwrap();
    println!("{}", String::from_utf8(whoami.stdout).unwrap());

    session.close().await.unwrap();
}
