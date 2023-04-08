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
    return match target {
        TargetType::EnvTest => spawn_envtest(ssh_conns, config).await,
        TargetType::EnvTestGeo => spawn_envtest_geo(ssh_conns, config).await,
        TargetType::HotStuff => spawn_hotstuff(ssh_conns, config).await,
        TargetType::HotStuffBumped => spawn_hotstuff_bumped(ssh_conns, config).await,
        TargetType::Pompe => spawn_pompe(ssh_conns, config).await,
        TargetType::PompeBumped => spawn_pompe_bumped(ssh_conns, config, false).await,
        TargetType::PompeUnbiasBumped => spawn_pompe_bumped(ssh_conns, config, true).await,
        _ => Err(OracleError::UnknownTarget)?
    }
}

pub async fn spawn_hotstuff<'a>(ssh_conns: &'a HashMap<String, Session>,
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
    println!("{} Spawn {} server processes on remote hosts.", "[5/7]".yellow(), &config["server-hosts"].len());
    let mut server_id = 0;
    for server in &config["server-hosts"] {
        let idx_arg = format!("{}{}{}", &config["server-idx-arg"][0], server_id, &config["server-idx-arg"][1]);

        match ssh_conns.get(server) {
            None => { Err(OracleError::InvalidServerHost)? }
            Some(s) => {
                process.push(s.command(server_cmd.as_str())
                             .args(&config["server-args"])
                             .arg(idx_arg)
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        server_id += 1;
    }
    thread::sleep(time::Duration::from_millis(1000));

    // Spawn client processes
    println!("{} Spawn {} client processes on remote hosts.", "[6/7]".yellow(), config["client-hosts"].len());
    let mut client_id = 0;
    for client in &config["client-hosts"] {
        // let latency = &latency_config[&host_to_location[client]]
        //                              [host_to_lidx[server]];
        //println!("From {} to {}: {}ms", client, server, latency);
        match ssh_conns.get(client) {
            None => { Err(OracleError::InvalidClientHost)? }
            Some(s) => {
                //println!("ssh: {} {:?} --idx {} --serveridx {} --latency {}", client_cmd, &config["client-args"], client_id, server_id, latency);
                //
                process.push(s.command(client_cmd.as_str())
                             .args(&config["client-args"])
                             .arg("--cid")
                             .arg(client_id.to_string())
                             // .arg("--latency")
                             // .arg(latency.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        client_id += 1;
    }

    Ok(process)
}

pub async fn spawn_hotstuff_bumped<'a>(ssh_conns: &'a HashMap<String, Session>,
                                       config: &'a HashMap<String, Vec<String>>
) -> Result<Vec<RemoteChild<'a>>, OracleError> {
    let mut process = vec![];
    // process will be returned and its lifetime (e.g., the lifetime of
    // the remote processes) should continue after this function returns

    let binary_dir = &config["remote-dir"][0];
    let client_bin = &config["binary-files"][0];
    let server_bin = &config["binary-files"][1];
    let bumper_bin = &config["binary-files"][2];
    let client_cmd = format!("{}{}", binary_dir, client_bin);
    let server_cmd = format!("{}{}", binary_dir, server_bin);
    let bump_cmd = format!("{}{}", binary_dir, bumper_bin);

    // Spawn server processes
    println!("{} Spawn {} server processes on remote hosts.", "[5/7]".yellow(), &config["server-hosts"].len());
    let mut server_id = 0;
    for server in &config["server-hosts"] {
        let idx_arg = format!("{}{}{}", &config["server-idx-arg"][0], server_id, &config["server-idx-arg"][1]);

        match ssh_conns.get(server) {
            None => { Err(OracleError::InvalidServerHost)? }
            Some(s) => {
                process.push(s.command(server_cmd.as_str())
                             .args(&config["server-args"])
                             .arg(idx_arg)
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        server_id += 1;
    }
    thread::sleep(time::Duration::from_millis(1000));

    // Spawn speedbump processes
    println!("{} Spawn {} speedbump processes on remote hosts.", "[5.5/7]".yellow(), &config["bump-hosts"].len());
    let mut bump_id = 0;
    for speedbump in &config["bump-hosts"] {
        //let idx_arg = format!("{}{}{}", &config["bump-idx-arg"][0], bump_id, &config["bump-idx-arg"][1]);
        match ssh_conns.get(speedbump) {
            None => { Err(OracleError::InvalidBumpHost)? }
            Some(s) => {
                process.push(s.command(bump_cmd.as_str())
                             .args(&config["bump-args"])
                             //.arg(idx_arg)
                             .arg("--idx")
                             .arg(bump_id.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        bump_id += 1;
    }
    thread::sleep(time::Duration::from_millis(1000));
    
    // Spawn client processes
    println!("{} Spawn {} client processes on remote hosts.", "[6/7]".yellow(), config["client-hosts"].len());
    let mut client_id = 0;
    for client in &config["client-hosts"] {
        // let latency = &latency_config[&host_to_location[client]]
        //                              [host_to_lidx[server]];
        //println!("From {} to {}: {}ms", client, server, latency);
        match ssh_conns.get(client) {
            None => { Err(OracleError::InvalidClientHost)? }
            Some(s) => {
                //println!("ssh: {} {:?} --idx {} --serveridx {} --latency {}", client_cmd, &config["client-args"], client_id, server_id, latency);
                //
                process.push(s.command(client_cmd.as_str())
                             .args(&config["client-args"])
                             .arg("--cid")
                             .arg(client_id.to_string())
                             // .arg("--latency")
                             // .arg(latency.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        client_id += 1;
    }

    Ok(process)
}


pub async fn spawn_pompe<'a>(ssh_conns: &'a HashMap<String, Session>,
                             config: &'a HashMap<String, Vec<String>>
) -> Result<Vec<RemoteChild<'a>>, OracleError> {
    let mut process = vec![];
    // process will be returned and its lifetime (e.g., the lifetime of
    // the remote processes) should continue after this function returns

    let log_dir = &config["log-dir"][0];
    let binary_dir = &config["remote-dir"][0];
    let client_bin = &config["binary-files"][0];
    let server_bin = &config["binary-files"][1];
    let client_cmd = format!("{}{}", binary_dir, client_bin);
    let server_cmd = format!("{}{}", binary_dir, server_bin);

    // Spawn server processes
    println!("{} Spawn {} server processes on remote hosts.", "[5/7]".yellow(), &config["server-hosts"].len());
    let mut server_id = 0;
    for server in &config["server-hosts"] {
        let log_arg = format!("{}server{}.log", log_dir, server_id);
        let idx_arg = format!("{}{}{}", &config["server-idx-arg"][0], server_id, &config["server-idx-arg"][1]);

        match ssh_conns.get(server) {
            None => { Err(OracleError::InvalidServerHost)? }
            Some(s) => {
                process.push(s.command(server_cmd.as_str())
                             .args(&config["server-args"])
                             .arg(log_arg)
                             .arg("--conf")
                             .arg(idx_arg)
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        server_id += 1;
    }
    thread::sleep(time::Duration::from_millis(1000));

    // Spawn client processes
    println!("{} Spawn {} client processes on remote hosts.", "[6/7]".yellow(), config["client-hosts"].len());
    let mut client_id = 0;
    for client in &config["client-hosts"] {
        let orderlog_arg = format!("{}client{}.order.log", log_dir, server_id);
        let execlog_arg = format!("{}client{}.exec.log", log_dir, server_id);
        // let latency = &latency_config[&host_to_location[client]]
        //                              [host_to_lidx[server]];
        //println!("From {} to {}: {}ms", client, server, latency);
        match ssh_conns.get(client) {
            None => { Err(OracleError::InvalidClientHost)? }
            Some(s) => {
                //println!("ssh: {} {:?} --idx {} --serveridx {} --latency {}", client_cmd, &config["client-args"], client_id, server_id, latency);
                //
                process.push(s.command(client_cmd.as_str())
                             .args(&config["client-args"])
                             .arg(orderlog_arg)
                             .arg(execlog_arg)
                             .arg("--cid")
                             .arg(client_id.to_string())
                             // .arg("--latency")
                             // .arg(latency.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        client_id += 1;
    }

    Ok(process)
}

pub async fn spawn_pompe_bumped<'a>(ssh_conns: &'a HashMap<String, Session>,
                                    config: &'a HashMap<String, Vec<String>>,
                                    unbias: bool
) -> Result<Vec<RemoteChild<'a>>, OracleError> {
    let mut process = vec![];
    // process will be returned and its lifetime (e.g., the lifetime of
    // the remote processes) should continue after this function returns

    let log_dir = &config["log-dir"][0];
    let binary_dir = &config["remote-dir"][0];
    let client_bin = &config["binary-files"][0];
    let server_bin = &config["binary-files"][1];
    let bumper_bin = &config["binary-files"][2];
    let tc_script = &config["binary-files"][3];
    let client_cmd = format!("{}{}", binary_dir, client_bin);
    let server_cmd = format!("{}{}", binary_dir, server_bin);
    let bump_cmd = format!("{}{}", binary_dir, bumper_bin);
    let tc_cmd = format!("{}{}", binary_dir, tc_script);

    // Spawn server processes
    println!("{} Spawn {} server processes on remote hosts.", "[5/7]".yellow(), &config["server-hosts"].len());
    let mut server_id = 0;
    for server in &config["server-hosts"] {
        let log_arg = format!("{}server{}.log", log_dir, server_id);
        let idx_arg = format!("{}{}{}", &config["server-idx-arg"][0], server_id, &config["server-idx-arg"][1]);

        match ssh_conns.get(server) {
            None => { Err(OracleError::InvalidServerHost)? }
            Some(s) => {
                process.push(s.command(server_cmd.as_str())
                             .args(&config["server-args"])
                             .arg(log_arg)
                             .arg("--conf")
                             .arg(idx_arg)
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        server_id += 1;
    }
    thread::sleep(time::Duration::from_millis(500));

    // Spawn strong speedbump processes
    println!("{} Spawn {} strong speedbump processes on remote hosts.", "(1/2)".yellow(), &config["strong-bump-hosts"].len());
    let mut bump_id = 0;
    for speedbump in &config["strong-bump-hosts"] {
        //let idx_arg = format!("{}{}{}", &config["bump-idx-arg"][0], bump_id, &config["bump-idx-arg"][1]);
        let latency = &config["strong-bump-latency"][bump_id];
        match ssh_conns.get(speedbump) {
            None => { Err(OracleError::InvalidBumpHost)? }
            Some(s) => {
                // Setup network latency
                s.command(tc_cmd.as_str())
                    .arg(latency)
                    .output()
                    .await
                    .map_err(|_| OracleError::SshCommandFailed)?;
                // Spawn speedbump process
                process.push(s.command(bump_cmd.as_str())
                             .args(&config["strong-bump-args"])
                             //.arg(idx_arg)
                             .arg("--idx")
                             .arg(bump_id.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        bump_id += 1;
    }
    thread::sleep(time::Duration::from_millis(500));

    // Spawn weak speedbump processes
    println!("{} Spawn {} weak speedbump processes on remote hosts.", "(2/2)".yellow(), &config["weak-bump-hosts"].len());
    let mut bump_id = 0;
    for speedbump in &config["weak-bump-hosts"] {
        //let idx_arg = format!("{}{}{}", &config["bump-idx-arg"][0], bump_id, &config["bump-idx-arg"][1]);
        let latency = &config["weak-bump-latency"][bump_id];
        match ssh_conns.get(speedbump) {
            None => { Err(OracleError::InvalidBumpHost)? }
            Some(s) => {
                // Setup network latency
                s.command(tc_cmd.as_str())
                    .arg(latency)
                    .output()
                    .await
                    .map_err(|_| OracleError::SshCommandFailed)?;
                // Spawn speedbump process
                process.push(s.command(bump_cmd.as_str())
                             .args(&config["weak-bump-args"])
                             //.arg(idx_arg)
                             .arg("--idx")
                             .arg(bump_id.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        bump_id += 1;
    }
    thread::sleep(time::Duration::from_millis(500));

    // Spawn strong client process
    println!("{} Spawn strong/weak client processes on remote hosts.", "[6/7]".yellow());
    let mut client_id = 0;
    let weak_client = &config["client-hosts"][0];
    let strong_client = &config["client-hosts"][1];
    let orderlog_arg = format!("{}client{}.order.log", log_dir, server_id);
    let execlog_arg = format!("{}client{}.exec.log", log_dir, server_id);
    // let latency = &latency_config[&host_to_location[client]]
    //                              [host_to_lidx[server]];
    //println!("From {} to {}: {}ms", client, server, latency);

    if unbias {
        // Pompe unbias client
        match ssh_conns.get(weak_client) {
            None => { Err(OracleError::InvalidClientHost)? }
            Some(s) => {
                //println!("ssh: {} {:?} --idx {} --serveridx {} --latency {}", client_cmd, &config["client-args"], client_id, server_id, latency);
                process.push(s.command(client_cmd.as_str())
                             .args(&config["client-args"])
                             .arg(&orderlog_arg)
                             .arg(&execlog_arg)
                             .arg("--cid")
                             .arg(client_id.to_string())
                             // .arg("--latency")
                             // .arg(latency.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
    } else {
        // Pompe original client
        match ssh_conns.get(weak_client) {
            None => { Err(OracleError::InvalidClientHost)? }
            Some(s) => {
                //println!("ssh: {} {:?} --idx {} --serveridx {} --latency {}", client_cmd, &config["client-args"], client_id, server_id, latency);
                process.push(s.command(client_cmd.as_str())
                             .args(&config["weak-client-args"])
                             .arg(&orderlog_arg)
                             .arg(&execlog_arg)
                             .arg("--cid")
                             .arg(client_id.to_string())
                             // .arg("--latency")
                             // .arg(latency.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }

        client_id += 1;
        match ssh_conns.get(strong_client) {
            None => { Err(OracleError::InvalidClientHost)? }
            Some(s) => {
                //println!("ssh: {} {:?} --idx {} --serveridx {} --latency {}", client_cmd, &config["client-args"], client_id, server_id, latency);
                //
                process.push(s.command(client_cmd.as_str())
                             .args(&config["strong-client-args"])
                             .arg(&orderlog_arg)
                             .arg(&execlog_arg)
                             .arg("--cid")
                             .arg(client_id.to_string())
                             // .arg("--latency")
                             // .arg(latency.to_string())
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
    }

    Ok(process)
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
    println!("{} Spawn {} server processes on remote hosts.", "[5/7]".yellow(), &config["server-hosts"].len());
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
    println!("{} Spawn {} client processes on remote hosts.", "[6/7]".yellow(), config["client-hosts"].len() * config["server-hosts"].len());
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

    Ok(process)
}

pub async fn spawn_envtest_geo<'a>(ssh_conns: &'a HashMap<String, Session>,
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
    println!("{} Spawn {} server processes on remote hosts.", "[5/7]".yellow(), &config["server-hosts"].len());
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
    thread::sleep(time::Duration::from_millis(1000));

    // Spawn client processes
    println!("{} Spawn {} client processes on remote hosts.", "[6/7]".yellow(), config["client-hosts"].len());

    let mut client_id = 0;
    let mut server_id = 0;
    for client in &config["client-hosts"] {
        let latency = &config["client-latencies"][client_id];
        match ssh_conns.get(client) {
            None => { Err(OracleError::InvalidClientHost)? }
            Some(s) => {
                // add network delay
                // sudo tc qdisc add dev enp1s0d1 root netem delay ??ms
                println!("[DEBUG] client {} latency {}ms", client, latency);
                s.command("sudo")
                    .args(["tc", "qdisc", "add", "dev", "enp1s0d1", "root", "netem", "delay"])
                    .arg(format!("{}ms", latency))
                    .output()
                    .await
                    .map_err(|_| OracleError::SshCommandFailed)?;

                process.push(s.command(client_cmd.as_str())
                             .args(&config["client-args"])
                             .arg("--idx")
                             .arg(client_id.to_string())
                             .arg("--serveridx")
                             .arg(server_id.to_string())
                             .arg("--latency")
                             .arg(latency)
                             .spawn()
                             .await
                             .map_err(|_| OracleError::SshCommandFailed)?
                );
            }
        }
        client_id += 1;
        server_id = (server_id + 1) % config["server-hosts"].len();
    }

    Ok(process)
}
