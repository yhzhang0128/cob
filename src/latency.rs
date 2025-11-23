use crate::error::OracleError;
use std::collections::HashMap;
use crate::ssh::start_ssh_conns;


static HOSTS: [&'static str; 12] = ["host0", "host1", "host2", "host3",
                                    "host4", "host5", "host6", "host7",
                                    "host8", "host9", "host10", "host11"];

// The command below can get the NIC interface name with IP address 10.*.*.*
// using the ifconfig command. It works on CloudLab, but not tested elsewhere.
static NIC: &str = "ifconfig | grep -B1 \"inet 10\\.\" | head -n 1 | grep -o '^[^:]*'";

pub async fn remove_latency() -> Result<(), OracleError>  {

    let hosts: Vec<String> = HOSTS.iter().map( |i| i.to_string() ).collect();
    let ssh_conns = start_ssh_conns(&hosts).await?;

    for (host, s) in &ssh_conns {
        println!("Removing latency on {}", host);

        s.raw_command(format!("sudo tc qdisc del dev `{}` root", NIC))
         .output()
         .await
         .map_err(|_| OracleError::SshCommandFailed)?;
    }

    Ok(())
}

static AMSTERDAM:    [u32; 12] = [0,   119, 281, 9,   15,  36,  10,  142, 172, 270, 91,  81 ];
static AUSTIN:       [u32; 12] = [119, 0,   190, 111, 126, 150, 114, 43,  274, 138, 41,  54 ];
static CANBERRA:     [u32; 12] = [281, 190, 0,   275, 276, 296, 278, 156, 99,  221, 235, 206];
static LONDON:       [u32; 12] = [9,   115, 275, 0,   20,  40,  9,   137, 172, 243, 86,  76 ];
static MUNICH:       [u32; 12] = [15,  126, 275, 34,  0,   41,  16,  158, 178, 220, 109, 92 ];
static OULU:         [u32; 12] = [36,  153, 288, 40,  41,  0,   46,  170, 187, 274, 121, 112];
static PARIS:        [u32; 12] = [11,  114, 269, 8,   16,  45,  0,   146, 152, 272, 91,  78 ];
static SANFRANCISCO: [u32; 12] = [142, 42,  156, 137, 158, 170, 146, 0,   223, 107, 60,  71 ];
static SINGAPORE:    [u32; 12] = [142, 42,  156, 137, 158, 170, 146, 0,   223, 107, 60,  71 ];
static TOKYO:        [u32; 12] = [271, 139, 221, 243, 220, 274, 272, 107, 83,  0,   154, 163];
static TORONTO:      [u32; 12] = [91,  41,  235, 87,  107, 121, 91,  62,  282, 154, 0,   71 ];
static WASHINGTON:   [u32; 12] = [81,  54,  206, 76,  92,  111, 78,  71,  270, 163, 71,  0  ];

pub async fn setup_latency() -> Result<(), OracleError>  {

    let hosts: Vec<String> = HOSTS.iter().map( |i| i.to_string() ).collect();
    let ssh_conns = start_ssh_conns(&hosts).await?;
    let mut latency_map = HashMap::new();

    latency_map.insert(String::from("host0"), AMSTERDAM);
    latency_map.insert(String::from("host1"), AUSTIN);
    latency_map.insert(String::from("host2"), CANBERRA);
    latency_map.insert(String::from("host3"), LONDON);
    latency_map.insert(String::from("host4"), MUNICH);
    latency_map.insert(String::from("host5"), OULU);
    latency_map.insert(String::from("host6"), PARIS);
    latency_map.insert(String::from("host7"), SANFRANCISCO);
    latency_map.insert(String::from("host8"), SINGAPORE);
    latency_map.insert(String::from("host9"), TOKYO);
    latency_map.insert(String::from("host10"), TORONTO);
    latency_map.insert(String::from("host11"), WASHINGTON);

    for (host, s) in &ssh_conns {
        if !latency_map.contains_key(host) {
            Err(OracleError::LatencyMapIncomplete)?
        }
        let latencies = latency_map.get(host).unwrap();
        println!("Setting up latency on {}: {:?}", host, latency_map.get(host));

        s.raw_command(format!("sudo tc qdisc del dev `{}` root", NIC))
         .output()
         .await
         .map_err(|_| OracleError::SshCommandFailed)?;

        s.raw_command(format!("sudo tc qdisc add dev `{}` root handle 1: htb", NIC))
         .output()
         .await
         .map_err(|_| OracleError::SshCommandFailed)?;

        s.raw_command(format!("sudo tc class add dev `{}` parent 1: classid 1:1 htb rate 10gibps", NIC))
         .output()
         .await
         .map_err(|_| OracleError::SshCommandFailed)?;

        for idx in 0..latencies.len() {
            //println!("{}", format!("sudo tc class add dev `{}` parent 1:1 classid 1:{} htb rate 1gibps", NIC, idx+2));
            s.raw_command(format!("sudo tc class add dev `{}` parent 1:1 classid 1:{} htb rate 1gibps", NIC, idx+2))
             .output()
             .await
             .map_err(|_| OracleError::SshCommandFailed)?;
            //println!("{}", format!("sudo tc qdisc add dev `{}` handle {}: parent 1:{} netem delay {}ms", NIC, idx+2, idx+2, latencies[idx]));
            s.raw_command(format!("sudo tc qdisc add dev `{}` handle {}: parent 1:{} netem delay {}ms", NIC, idx+2, idx+2, latencies[idx]))
             .output()
             .await
             .map_err(|_| OracleError::SshCommandFailed)?;
            //println!("{}", format!("sudo tc filter add dev `{}` parent 1: protocol ip u32 match ip dst `dig +short host{}-link-0`/32 flowid 1:{};", NIC, idx, idx+2));
            s.raw_command(format!("sudo tc filter add dev `{}` parent 1: protocol ip u32 match ip dst `dig +short host{}-link-0`/32 flowid 1:{};", NIC, idx, idx+2))
             .output()
             .await
             .map_err(|_| OracleError::SshCommandFailed)?;
        }
    }

    Ok(())
}
