use std::time;
use std::thread;
use colored::Colorize;
use std::process::Command;
use indicatif::ProgressBar;

use crate::kill::killall;
use crate::cli::TargetType;
use crate::error::OracleError;

use crate::ssh::{
    start_ssh_conns,
    close_ssh_conns,
};
use crate::prep::prepare_files;
use crate::spawn::spawn_target;
use crate::config::read_config;

pub async fn evaluate(target: &TargetType, duration: u64) -> Result<(), OracleError>{
    println!("{}", format!("Target: {:?}", target).green().bold());
    println!("{}", format!("Duration: {:?}ms", duration).green().bold());

    let config = read_config(&target)?;
    // Build the target
    println!("{} Build target {:?} with {}.", "[1/7]".yellow(), target, config["build"][0]);
    let status = Command::new("bash")
        .args(&config["build"])
        .status()
        .expect("Failed to build the target.");
    assert!(status.success());
    
    // Start ssh connections
    println!("{} Start ssh connections to {} remote hosts.", "[2/7]".yellow(), config["hostnames"].len());
    let ssh_conns = start_ssh_conns(&config["hostnames"]).await?;

    // Prepare the directories and binary files
    prepare_files(&ssh_conns, &config).await?;

    // Spawn server and client processes on remote machines
    let processes = spawn_target(target, &ssh_conns, &config).await?;

    // Wait a duration and terminate the experiment
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(time::Duration::from_millis(120));
    let msg = format!("Executing remote client/server for {}ms.", duration);
    pb.set_message(msg);
    thread::sleep(time::Duration::from_millis(duration));
    let finish_msg = format!("Terminate experiment after {}ms.", duration);
    pb.finish_with_message(finish_msg);

    // Collect output and close connections
    println!("{} Collect output from {} processes and close ssh connections.", "[7/7]".yellow(), processes.len());
    killall(&target, false).await?;
    thread::sleep(time::Duration::from_millis(5000));

    close_ssh_conns(ssh_conns).await?;
    Ok(())
}
