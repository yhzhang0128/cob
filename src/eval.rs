use std::time;
use std::thread;
use colored::Colorize;
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
use crate::config::read_host_config;

pub async fn evaluate(target: TargetType, duration: u64) -> Result<(), OracleError>{
    println!("{}", format!("Target: {:?}", target).green().bold());
    println!("{}", format!("Duration: {:?}ms", duration).green().bold());

    // Start ssh connections
    let host_config = read_host_config()?;
    println!("{} Start ssh connections to {} remote hosts.", "[1/6]".yellow(), host_config["hostnames"].len());
    let ssh_conns = start_ssh_conns(&host_config["hostnames"]).await?;

    // Prepare the directories and binary files
    prepare_files(&ssh_conns, &host_config).await?;

    // Spawn server and client processes on remote machines
    spawn_target(target, &ssh_conns, &host_config).await?;

    // Wait a duration and terminate the experiment
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(time::Duration::from_millis(120));
    let msg = format!("Executing remote client/server for {}ms.", duration);
    pb.set_message(msg);
    thread::sleep(time::Duration::from_millis(duration));
    let finish_msg = format!("Terminate experiment after {}ms.", duration);
    pb.finish_with_message(finish_msg);

    // Collect output and close connections
    println!("{} Collect output and close ssh connections.", "[6/6]".yellow());
    killall(false).await?;
    close_ssh_conns(ssh_conns).await?;

    Ok(())
}
