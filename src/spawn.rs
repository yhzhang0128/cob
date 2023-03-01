use std::time;
use std::thread;
use std::collections::HashMap;
use openssh::process::RemoteChild;

use openssh::Session;
use colored::Colorize;
use crate::cli::TargetType;
use crate::error::OracleError;
use crate::config::read_latency_config;

pub async fn spawn_target<'a>(target: &TargetType,
                              ssh_conns: &'a HashMap<String, Session>,
                              config: &'a HashMap<String, Vec<String>>
) -> Result<Vec<RemoteChild<'a>>, OracleError> {
    match target {
        TargetType::EnvTest => { return spawn_envtest(ssh_conns, config).await; }
        TargetType::HotStuff => { Err( OracleError::NotImplemented )?; }
        TargetType::Pompe => { Err( OracleError::NotImplemented )?; }
        _ => { Err(OracleError::UnknownTarget)?; }
    }

    Err(OracleError::UnknownTarget)?
}

pub async fn spawn_envtest<'a>(ssh_conns: &'a HashMap<String, Session>,
                               config: &'a HashMap<String, Vec<String>>
) -> Result<Vec<RemoteChild<'a>>, OracleError> {
    let mut process = vec![];
    // process will be returned and its lifetime (e.g., the lifetime of
    // the remote processes) should continue after this function returns

    let binary_dir = &config["remote-dir"][0];
    let client_bin = &config["binary-files"][0];
    let server_bin = &config["binary-files"][1];
    let client_cmd = format!("{}{}", binary_dir, client_bin);
    let server_cmd = format!("{}{}", binary_dir, server_bin);

    // Spawn server processes
    let mut server_id = 0;
    for server in &config["server-hosts"] {
        match ssh_conns.get(server) {
            None => { Err(OracleError::InvalidServerHost)? }
            Some(s) => {
                process.push(s.command(server_cmd.as_str())
                             .args(&config["server-args"])
                             .arg("--idx")
                             .arg(server_id.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        server_id += 1;
    }
    println!("{} Execute {} server processes on remote hosts.", "[5/7]".yellow(), server_id);
    thread::sleep(time::Duration::from_millis(1000));

    // Create geo-location latency mapping
    let latency_config = read_latency_config()?;
    let mut idx : usize = 0;
    let mut location_to_idx = HashMap::<String, usize>::new();
    for l in &latency_config["locations"] {
        location_to_idx.insert(l.to_string(), idx);
        idx += 1;
    }
    idx = 0;
    let mut host_to_lidx = HashMap::<String, usize>::new();
    let mut host_to_location = HashMap::<String, String>::new();
    for h in &config["hostnames"] {
        let l = &config["locations"][idx];
        host_to_location.insert(h.to_string(), l.to_string());
        host_to_lidx.insert(h.to_string(), location_to_idx[l]);
        idx += 1;
    }
    
    // Spawn client processes
    println!("Here: {:?} || {:?}", &config["client-hosts"], &config["server-hosts"]);
    let mut client_id = 0;
    for client in &config["client-hosts"] {
        let mut server_id = 0;
        for server in &config["server-hosts"] {
            let latency = &latency_config[&host_to_location[client]]
                                         [host_to_lidx[server]];
            //println!("From {} to {}: {}ms", client, server, latency);
            match ssh_conns.get(client) {
                None => { Err(OracleError::InvalidClientHost)? }
                Some(s) => {
                    //println!("ssh: {} {:?} --idx {} --serveridx {} --latency {}", client_cmd, &config["client-args"], client_id, server_id, latency);
                    process.push(s.command(client_cmd.as_str())
                                 .args(&config["client-args"])
                                 .arg("--idx")
                                 .arg(client_id.to_string())
                                 .arg("--serveridx")
                                 .arg(server_id.to_string())
                                 .arg("--latency")
                                 .arg(latency.to_string())
                                 .spawn()
                                 .await
                                 .map_err(|_| OracleError::SshCommandFailed)?
                    );
                }
            }
            server_id += 1;
        }
        client_id += 1;
    }
    println!("{} Execute {} client processes on remote hosts.", "[6/7]".yellow(), config["client-hosts"].len() * config["server-hosts"].len());

    Ok(process)
}
