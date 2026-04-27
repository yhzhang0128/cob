# Benchmark for Equal Opportunity in Ordered Consensus

This benchmark toolkit named Chance Oracle Benchmark (COB) is developed for paper [**Equal Opportunity: A Correctness Condition for Ordered Consensus**](https://www.usenix.org/conference/osdi26/presentation/zhang-yunhao) at OSDI'26.

## Claims, Figures, and Experiments

There are 8 experiments leading to the 5 figures in the evaluation section of this paper.
This table summarizes the claims supported by these figures and experiments.

| Figures | Experiments  | Claims                                                                                                 |
|---------|--------------|--------------------------------------------------------------------------------------------------------|
| Fig. 8  | Exp. 1, 2, 3 | The 3 baseline systems are all vulnerable to bias.                                                     |
| Fig. 9  | Exp. 4       | Pompe-SRO can control the degree of bias under a target Ɛ.                                             |
| Fig. 10 | Exp. 5       | An attacker gets the mean reward in Pompe-SRO and the max in baselines.                                |
| Fig. 11 | Exp. 6       | SRO latency is low and SRO is not the performance bottleneck in Pompe-SRO.                             |
| Fig. 12 | Exp. 7, 8    | Pompe-SRO maintains the same end-to-end throughput as Pompe and incurs a latency overhead of 47%-67%.  |

The two sections below help you setup the environment and provide detailed instructions for the 8 experiments.

## Getting Started Instructions

### Setup

You need 13 machines to run the experiments.
A **control** machine runs the COB code instrumenting the experiments.
Each of the other 12 machines represents a geolocation, and we call them **host0**, **host1**, ..., **host11**.
If you have a CloudLab account, two CloudLab profiles have been defined for you: [ChanceOracle-emulab](https://www.cloudlab.us/show-profile.php?uuid=19e42112-bf1d-11f0-90d9-e4434b2381fc) and [ChanceOracle-emulab-8core](https://www.cloudlab.us/show-profile.php?uuid=2a39d76e-bf1f-11f0-bc80-e4434b2381fc).
The first profile uses the `d710` machine which is less powerful and enough for measuring bias.
The second profile uses the `d430` machine which is needed for measuring performance.
We suggest that you start with a CloudLab account and reuse these CloudLab profiles.
If you have to use your own server machines, we provide some instructions for modifying COB at the end of this README.

### Clone and Build the Code

In the **control** machine, clone the COB repo to your work directory `$WORKDIR`.

```console
> cd $WORKDIR
> git clone git@github.com:yhzhang0128/cob.git
> cd $WORKDIR/cob
> git submodule update --init --recursive
```

[Install Rust](https://rust-lang.org/tools/install/), and build the COB code.

```console
> cd $WORKDIR/cob
> cargo build
...
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
```

There are 2 submodules in COB: `target_systems/hotstuff` and `target_systems/pompe`.
As mentioned in [this README](https://github.com/Pompe-org/libhotstuff/blob/master/README.rst), you can install the dependencies with `sudo apt-get install libssl-dev libuv1-dev cmake make`.
Note that you need to install these dependencies on **all the 13 machines**.
You can then compile the binaries of HotStuff and Pompe:

```console
> cd $WORKDIR/cob/target_systems/hotstuff
> ./build.sh
...
[ 83%] Built target hotstuff-app
[ 89%] Built target hotstuff-client
[ 94%] Built target hotstuff-keygen
[100%] Built target test_secp256k1
> cd $WORKDIR/cob/target_systems/pompe
> ./build.sh
[ 95%] Built target pompe-client
[ 95%] Built target hotstuff-app
[100%] Built target pompe-app
```

Make sure that you succeed in compiling HotStuff and Pompe before you proceed.
If there's an error, you may need to search a bit and install some missing dependencies.
Both Pompe and Pompe-SRO are in the `target_systems/pompe` submodule, and they are in different git branches.

### Test the Latency Emulation

In `src/latency.rs`, you can find the following 12 lines of code:

```rust
static AMSTERDAM:    [u32; 12] = [0,   119, 281, 9,   15,  36,  10,  142, 172, 270, 91,  81 ];
static AUSTIN:       [u32; 12] = [119, 0,   190, 111, 126, 150, 114, 43,  274, 138, 41,  54 ];
static CANBERRA:     [u32; 12] = [281, 190, 0,   275, 276, 296, 278, 156, 99,  221, 235, 206];
static LONDON:       [u32; 12] = [9,   115, 275, 0,   20,  40,  9,   137, 172, 243, 86,  76 ];
static MUNICH:       [u32; 12] = [15,  126, 275, 34,  0,   41,  16,  158, 178, 220, 109, 92 ];
static OULU:         [u32; 12] = [36,  153, 288, 40,  41,  0,   46,  170, 187, 274, 121, 112];
static PARIS:        [u32; 12] = [11,  114, 269, 8,   16,  45,  0,   146, 152, 272, 91,  78 ];
static SANFRANCISCO: [u32; 12] = [142, 42,  156, 137, 158, 170, 146, 0,   223, 107, 60,  71 ];
static SINGAPORE:    [u32; 12] = [172, 274, 99,  172, 178, 187, 152, 223, 0,   83,  282, 270];
static TOKYO:        [u32; 12] = [271, 139, 221, 243, 220, 274, 272, 107, 83,  0,   154, 163];
static TORONTO:      [u32; 12] = [91,  41,  235, 87,  107, 121, 91,  62,  282, 154, 0,   71 ];
static WASHINGTON:   [u32; 12] = [81,  54,  206, 76,  92,  111, 78,  71,  270, 163, 71,  0  ];
```

Each line contains an array of 12 integers.
This is the latency table, e.g., the latency from Amsterdam to Austin is 119ms.
We map the 12 lines to **host0**..**host11** (i.e., **host0** is for Amsterdam and **host1** is for Austin).
You can setup the latency with cob:

```console
> cd $WORKDIR/cob
> cargo run latency
...
Setting up latency on host5: Some([36, 153, 288, 40, 41, 0, 46, 170, 187, 274, 121, 112])
...
Setting up latency on host10: Some([91, 41, 235, 87, 107, 121, 91, 62, 282, 154, 0, 71])
```

After `cargo run latency`, you can test whether latency emulation has been setup properly:

```console
> ssh host0
> ping host1
PING host1-link-0 (10.10.1.2) 56(84) bytes of data.
64 bytes from host1-link-0 (10.10.1.2): icmp_seq=1 ttl=64 time=238 ms
64 bytes from host1-link-0 (10.10.1.2): icmp_seq=2 ttl=64 time=238 ms
64 bytes from host1-link-0 (10.10.1.2): icmp_seq=3 ttl=64 time=238 ms
...
```

The ping latency is 119*2=238ms because it is a round trip between **host0** and **host1**.
Before running an experiment, make sure that latency emulation has been setup properly.

### Understand the TOML configuration



## Detailed Instructions

### Run Experiment #1

### Run Experiment #8

### Use a non-CloudLab Cluster
