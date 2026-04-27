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
If there's an error, you may need to search a bit and install missing dependencies.
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

### Understand the Process of Running an Experiment

We now go through `config/pompe.toml` and help you understand the process of running an experiment.

The `build = ["script/build_pompe_bias.sh"]` in `pompe.toml` means that COB runs this script first to build the two binaries for Pompe.
With the binaries ready, COB copies the config and binary files to all the 12 machines in `hostnames` according to the following lines in `pompe.toml`.

```console
config-files = ["conf-pompe-12"]
binary-files = ["pompe-client", "pompe-app"]
local-dir = ["/opt/home/cob/target_systems/pompe/examples/", "/opt/home/cob/target_systems/pompe/examples/"]
remote-dir = ["/opt/home/target_binary/", "/opt/home/target_config/"]
```

Specifically, for config files, COB will copy `/opt/home/cob/target_systems/pompe/examples/conf-pompe-12` from **control** to `/opt/home/target_config/` on **host0**..**host11**.
For binary files, COB will copy the two files `/opt/home/cob/target_systems/pompe/examples/pompe-client` and `/opt/home/cob/target_systems/pompe/examples/pompe-app` from **control** to directory `/opt/home/target_binary/` on **host0**..**host11**.
You don't need to modify anything here if you are using the CloudLab profiles we provide.

After copying all the files, COB runs the server binary on all the machines in `server-hosts`, and runs the client binary on all the machines in `client-hosts`.
By default, COB runs an experiment for 30 seconds, and then kills the client/server processes on all the 12 machines.
The stdout and stderr output from all the client/server processes will then be printed in the shell of **control** before COB terminates.
Other than stdout/stderr printing, the `/users/Yunhao/log/` directory (i.e., `log-dir` in `pompe.toml`) on each of **host0**..**host11** holds log files by the client/server processes.
These log files contain more details of the experiment result.

## Detailed Instructions

### Run Experiment #1

Experiment #1 measures bias in HotStuff.
Read the following lines in `config/hotstuff.toml` first.
The `client-hosts` below shows that we are measuring bias between London and Washington.

```toml
# host3  London
# host4  Munich
# host9  Tokyo
# host11 Washington

client-hosts = ["host3", "host11"]
```

When measuring bias between other cities, update the two elements of `client-hosts` accordingly.
You need to run two commands in parallel on the **control** machine for Experiment #1.

#### Experiment #1 Command #1

```console
> cd $WORKDIR/cob
> python3 script/sync_send.py
Server is listening on control:30000...
```

The `sync_send.py` script will ensure that commands from the two different cities are invoked at the same time.
In CloudLab, the ping latency between **control** and **host3** (or **control** and **host11**) is about 0.1ms,
so we actually mean that two clients on **host3** and **host11** will invoke their command at roughly the same time
(i.e., the time difference is negligible compared with the network latency in the latency table).

#### Experiment #1 Command #2

```console
> cd $WORKDIR/cob
> cargo run eval -t hotstuff -d 80000
[1/7] Build target HotStuff with script/build_hotstuff_bias.sh.
...
[2/7] Start ssh connections to 12 remote hosts.
[3/7] Setup directories for log, binary and config files on remote hosts.
[4/7] Copy binary and config files to remote hosts.
[5/7] Spawn 12 server processes on remote hosts.
[6/7] Spawn 2 client processes on remote hosts.
  Terminate experiment after 80000ms.
[7/7] Kill 14 processes and collect output (may cause segfault during kill).
...
Server#8: client0=4, client1=0, skip=0
Server#3: client0=8, client1=0, skip=0
Server#9: client0=0, client1=3, skip=0
Server#10: client0=0, client1=3, skip=0
Server#11: client0=0, client1=11, skip=0
Server#5: client0=12, client1=0, skip=0
Server#4: client0=30, client1=0, skip=0
Server#2: client0=0, client1=6, skip=0
Server#1: client0=0, client1=22, skip=0
Server#6: client0=12, client1=0, skip=0
Server#0: client0=6, client1=0, skip=0
Server#7: client0=0, client1=13, skip=0
```

According to the latency table, `Server#8` represents Singapore,
and the `client0=4, client1=0` means that, during the experiment time period, there are 4 times when both clients invoke a command at the same time and the London client (i.e., `client-hosts[0]` in `hotstuff.toml`) is ordered earlier by a Singapore leader.
The `Pr[L ≺ W]` in Figure 8 is calculated by the sum of the first column (i.e., client0) divided by the sum of both columns.
Similarly, `Pr[W ≺ L]` is the sum of the second column divided by the sum of both.

The `-d 80000` above means that the experiment will last for 80000ms (i.e., 80 seconds).
You can run the experiment longer to get a better result.

### Run Experiment #2

Experiment #2 measures bias in Pompe.
Read the following lines in `config/pompe.toml` first.
The `client-hosts` shows that we are measuring bias between Munich and Tokyo.

```toml
# host3  London
# host4  Munich
# host9  Tokyo
# host11 Washington

# Pompe: 2 clients, measuring bias
client-hosts = ["host4", "host9"]
build = ["script/build_pompe_bias.sh"]
```

Note that there exist different possibilities for `client-hosts` and `build` in `pompe.toml`.
COB will run Experiment #2 when you only keep the ones above.

#### Experiment #2 Command #1

```console
> cd $WORKDIR/cob
> python3 script/sync_send.py
Server is listening on control:30000...
```

This is the same as Experiment #1.

#### Experiment #2 Command #2

```console
> cd $WORKDIR/cob
> cargo run eval -t pompe
...
[2/7] Start ssh connections to 12 remote hosts.
[3/7] Setup directories for log, binary and config files on remote hosts.
[4/7] Copy binary and config files to remote hosts.
[5/7] Spawn 12 server processes on remote hosts.
[6/7] Spawn 2 client processes on remote hosts.
  Terminate experiment after 30000ms.
...
[DEBUG] consensus 1 finalized -> [0, 7) curr_time=1777285635472357, elapsed=1777285635472ms
[DEBUG] consensus 5 finalized -> [7, 17) curr_time=1777285637478393, elapsed=2006ms
[DEBUG] consensus 9 finalized -> [17, 27) curr_time=1777285639560228, elapsed=2081ms
...
[DEBUG] consensus 57 finalized -> [137, 147) curr_time=1777285663607678, elapsed=2003ms
[DEBUG] consensus 61 finalized -> [147, 157) curr_time=1777285665611530, elapsed=2003ms
```

Since we run the experiment for 30 seconds, and run consensus every 2 second in Pompe.
You can see roughly 15 `consensus ? finalized` in the printing above.
To get the result of Experiment #2, we need to inspect the log files from the two clients.
This gives us a chance to touch log files.

#### Get the Result of Experiment #2

```console
> ssh host4
> head log/client0.order.log
idx=0, median=1777285632283963, latency=34606
idx=1, median=1777285632684635, latency=34530
idx=2, median=1777285633085445, latency=34490
idx=3, median=1777285633486292, latency=34571
idx=4, median=1777285633887068, latency=34545
idx=5, median=1777285634287941, latency=34589
idx=6, median=1777285634688674, latency=34513
idx=7, median=1777285635089485, latency=34499
idx=8, median=1777285635490346, latency=34563
idx=9, median=1777285635891156, latency=34557
> ssh host9
> head log/client1.order.log
idx=0, median=1777285632403845, latency=154430
idx=1, median=1777285632804524, latency=154411
idx=2, median=1777285633205343, latency=154409
idx=3, median=1777285633606140, latency=154403
idx=4, median=1777285634006930, latency=154404
idx=5, median=1777285634407780, latency=154420
idx=6, median=1777285634808583, latency=154412
idx=7, median=1777285635209309, latency=154412
idx=8, median=1777285635610199, latency=154404
idx=9, median=1777285636011039, latency=154407
```

The two log files are `client0.order.log` on `host4` and `client1.order.log` on `host9`.
Each `idx` means two clients invoking their command at the same time, and the `median` shows the median timestamp used to order their command in Pompe.
For each `idx`, the `median` of client1 is higher than client0, meaning that the Munich client (client0) is ordered before the Tokyo client (client1) all the time,
leading to the number `1` in Figure 8.
Again, you can run the same experiment for other city pairs by updating the `client-hosts` in `pompe.toml`.
The `latency` is just for debugging.

### Run Experiment #3

Experiment #2 measures bias in Themis.
You need to install `numpy`, `cycler`, and `networkx` for Python3.
They are required by the `themis_protocol()` function from [the simulation code of Themis](https://github.com/anonthemis/themis-src-anon/blob/main/Aequitas-hotstuff/simulations/adv_reorder.py).

```console
> sudo apt update && sudo apt install python3-numpy python3-cycler python3-networkx
> cd $WORKDIR/cob
> python3 script/themis_sim.py
Themis
Locatoin 1 < Location 2:  0
Location 2 < Location 1:  500
```

Note that `Location1` and `Location2` in [themis_sim.py](script/themis_sim.py) are `LONDON` and `WASHINGTON` respectively,
so the result above means that Themis is biased towards the Washington client all the time.
You can change the `Location1` and `Location2` variables, run the Python script again, and obtain the other results for Themis in Figure 8.

The `num_txs=1000` in `themis_sim.py` means that we are simulating 1000 transactions (commands) in total.
This is why each of the 2 clients invokes 500 commands in this experiment, and the number above is 500.

### Run Experiment #4

Experiment #2 measures bias in Pompe-SRO.
Make sure to do the following `build` in `config/pompe.toml`:

```toml
# host3  London
# host4  Munich
# host9  Tokyo
# host11 Washington

# Pompe-SRO: 2 clients, measuring bias
client-hosts = ["host4", "host9"]
build = ["script/build_pompe_sro_bias.sh"]
```

Again, we need to run two commands in parallel on the **control** machine.

#### Experiment #4 Command #1

```console
> cd $WORKDIR/cob
> python3 script/sync_send.py
Server is listening on control:30000...
```

This is the same as Experiment #1 and #2.

#### Experiment #4 Command #2

```
> cargo run eval -t pompe
...
[2/7] Start ssh connections to 12 remote hosts.
[3/7] Setup directories for log, binary and config files on remote hosts.
[4/7] Copy binary and config files to remote hosts.
[5/7] Spawn 12 server processes on remote hosts.
[6/7] Spawn 2 client processes on remote hosts.
  Terminate experiment after 30000ms.
[7/7] Kill 14 processes and collect output (may cause segfault during kill).
...
```

Just like Experiment #2, we need to inspect the log files on `host4` and `host9` for the experiment result.

#### Get the Result of Experiment #4

```console
> ssh host4
> head log/client0.order.log
idx=0, median=1777288690568095, noise=1383000, median-sent=34546
idx=1, median=1777288690968846, noise=777000, median-sent=34481
idx=2, median=1777288691369667, noise=1793000, median-sent=34446
idx=3, median=1777288691770571, noise=1386000, median-sent=34483
idx=4, median=1777288692171404, noise=719000, median-sent=34504
idx=5, median=1777288692572256, noise=575000, median-sent=34524
idx=6, median=1777288692973099, noise=498000, median-sent=34516
idx=7, median=1777288693373924, noise=1277000, median-sent=34536
idx=8, median=1777288693774744, noise=1627000, median-sent=34478
idx=9, median=1777288694175636, noise=1985000, median-sent=34523
> ssh host9
> head log/client1.order.log
idx=0, median=1777288690673146, noise=886000, median-sent=139500
idx=1, median=1777288691074033, noise=915000, median-sent=139582
idx=2, median=1777288691474857, noise=335000, median-sent=139550
idx=3, median=1777288691875678, noise=1290000, median-sent=139548
idx=4, median=1777288692276495, noise=788000, median-sent=139513
idx=5, median=1777288692677366, noise=1061000, median-sent=139549
idx=6, median=1777288693078220, noise=864000, median-sent=139547
idx=7, median=1777288693479087, noise=745000, median-sent=139570
idx=8, median=1777288693879896, noise=746000, median-sent=139545
idx=9, median=1777288694280737, noise=168000, median-sent=139544
```

Recall that Pompe-SRO orders the commands by `median+noise`.
For each `idx`, the `median` of client1 is roughly **105ms** higher than the `median` of `client0`,
and each `noise` is randomly chosen from `[0ms..2000ms]`.
The `Pr[M ≺ T]` in Figure 9 is thus the ratio of `idx` where client0 has a lower `median+noise`.
After calculating both `Pr[M ≺ T]` and `Pr[T ≺ M]`, we get the number 0.45 in Figure 9.
Run the same experiment for other city pairs by updating the `client-hosts` in `pompe.toml`.

### Run Experiment #5

Experiment #5 measures the effect of front-running in HotStuff, Pompe, Themis, and Pompe-SRO.

#### HotStuff, Pompe, and Pompe-SRO

Redo Experiments #1, #2, and #4 with 2 differences:

1. Put the two clients on the same host in `client-hosts` of `pompe.toml`.
2. Run `script/frontrun.py` instead of `script/sync_send.py`.

The `frontrun.py` allows the attacker to invoke its command 10ms earlier than the victim.
This does not make a big difference in Pompe-SRO, but HotStuff and Pompe would certainly order the attacker first.

#### Themis
Uncomment the following lines in `themis_sim.py`:

```python
ATTACKER = [25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25 ]
VICTIM   = [30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30 ]
Location1 = ATTACKER
Location2 = VICTIM
```

This allows the attacker to be 5ms faster than the victim to all the 12 cities, and Themis will be fully biased to the attacker as well.

### Run Experiment #6

Experiment #6 measures the performance of the two SRO implementations.

### Run Experiment #7

Experiment #7 measures the end-to-end performance of Pompe-SRO against Pompe.

### Run Experiment #8

Experiment #8 measures the end-to-end performance of HotStuff-SRO against HotStuff.
This experiment is not included in the submission version of the paper, and we are still working on it.

## Use a non-CloudLab Server Cluster
