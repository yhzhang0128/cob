use crate::error::OracleError;
use std::collections::HashMap;
use crate::ssh::start_ssh_conns;


static HOSTS: [&'static str; 3] = ["host0", "host1", "host2"];

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

        s.raw_command(format!("sudo tc qdisc add dev {} root handle 1: htb", NIC))
         .output()
         .await
         .map_err(|_| OracleError::SshCommandFailed)?;

        s.raw_command(format!("sudo tc class add dev {} parent 1: classid 1:1 htb rate 10gibps", NIC))
         .output()
         .await
         .map_err(|_| OracleError::SshCommandFailed)?;
    }

    Ok(())
}

static Amsterdam:    [u32; 12] = [0,   119, 281, 9,   15,  36,  10,  142, 172, 270, 91,  81 ];
static Austin:       [u32; 12] = [119, 0,   190, 111, 126, 150, 114, 43,  274, 138, 41,  54 ];
static Canberra:     [u32; 12] = [281, 190, 0,   275, 276, 296, 278, 156, 99,  221, 235, 206];
static London:       [u32; 12] = [9,   115, 275, 0,   20,  40,  9,   137, 172, 243, 86,  76 ];
static Munich:       [u32; 12] = [15,  126, 275, 34,  0,   41,  16,  158, 178, 220, 109, 92 ];
static Oulu:         [u32; 12] = [36,  153, 288, 40,  41,  0,   46,  170, 187, 274, 121, 112];
static Paris:        [u32; 12] = [11,  114, 269, 8,   16,  45,  0,   146, 152, 272, 91,  78 ];
static SanFrancisco: [u32; 12] = [142, 42,  156, 137, 158, 170, 146, 0,   223, 107, 60,  71 ];
static Singapore:    [u32; 12] = [142, 42,  156, 137, 158, 170, 146, 0,   223, 107, 60,  71 ];
static Tokyo:        [u32; 12] = [271, 139, 221, 243, 220, 274, 272, 107, 83,  0,   154, 163];
static Toronto:      [u32; 12] = [91,  41,  235, 87,  107, 121, 91,  62,  282, 154, 0,   71 ];
static Washington:   [u32; 12] = [81,  54,  206, 76,  92,  111, 78,  71,  270, 163, 71,  0  ];

pub async fn setup_latency() -> Result<(), OracleError>  {

    let hosts: Vec<String> = HOSTS.iter().map( |i| i.to_string() ).collect();
    let ssh_conns = start_ssh_conns(&hosts).await?;
    let mut latency_map = HashMap::new();

    latency_map.insert(String::from("host0"), Amsterdam);
    latency_map.insert(String::from("host1"), Austin);
    latency_map.insert(String::from("host2"), Canberra);

     for (host, s) in &ssh_conns {
         println!("Setting up latency on {}", host);

         println!("Latency: {:?}", latency_map.get(host));
    }


    Ok(())
}
