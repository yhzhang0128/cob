# Benchmark for Equal Opportunity in Ordered Consensus

This benchmark toolkit named Chance Oracle Benchmark (COB) is developed for paper: [**Equal Opportunity: A Correctness Condition for Ordered Consensus**](https://www.usenix.org/conference/osdi26/presentation/zhang-yunhao) at OSDI'26.

## Claims, Figures, and Experiments

There are 8 experiments in total leading to the 5 figures in the evaluation section of the paper.
This table summarizes the claims supported by these figures and experiments.

| Figures    | Experiments  | Claims                                                                                                 |
|------------|--------------|--------------------------------------------------------------------------------------------------------|
| Fig. 8     | Exp. 1, 2, 3 | The 3 baseline systems are all vulnerable to bias.                                                     |
| Fig. 9     | Exp. 4       | Pompe-SRO can control the degree of bias under a target Ɛ.                                             |
| Fig. 10    | Exp. 5       | An attacker gets the mean reward in Pompe-SRO and the max in baselines.                                |
| Fig. 11    | Exp. 6       | SRO latency is low and SRO is not the performance bottleneck in Pompe-SRO.                             |
| Fig. 12,13 | Exp. 7, 8    | Pompe-SRO maintains the same end-to-end throughput as Pompe and incurs a latency overhead of 47%-67%.  |

## Getting Started Instructions

### Setup

You need 13 machines to run the experiments.
A **control** machine runs the COB code instrumenting the experiments.
Each of the other 12 machines represents a geolocation, and we call them **host0**, **host1**, ..., **host11**.
If you have a CloudLab account, two CloudLab profiles have been defined for you: [ChanceOracle-emulab](https://www.cloudlab.us/show-profile.php?uuid=19e42112-bf1d-11f0-90d9-e4434b2381fc) and [ChanceOracle-emulab-8core](https://www.cloudlab.us/show-profile.php?uuid=2a39d76e-bf1f-11f0-bc80-e4434b2381fc).
The first profile uses the `d710` machine which is less powerful and good for measuring bias.
The second profile uses the `d430` machine which is needed for measuring the end-to-end performance.
We suggest that you start with a CloudLab account and reuse these CloudLab profiles.
If you have to use your own server cluster, we provide some instructions for modifying COB at the end of this README.

### Clone and Build the Code

In the CloudLab **control** machine, COB has been included in the disk image.

```console
> export WORKDIR=/opt/home
> cd $WORKDIR
> source env
> source .zshrc
> cd cob
```

If you use your own server cluster, clone COB to your work directory, and [install Rust](https://rust-lang.org/tools/install/).

```console
> cd $WORKDIR
> git clone git@github.com:yhzhang0128/cob.git
> cd $WORKDIR/cob
> git submodule update --init --recursive
```

Make sure that you can build the COB code.

```console
> cd $WORKDIR/cob
> cargo build
...
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
```

There are 3 submodules in COB: `target_systems/hotstuff`, `target_systems/pompe`, and `target_systems/tvrf`.
All dependencies have been installed for you in the CloudLab disk image, but if you are not using CloudLab,
you need to install the dependencies on all the machines in your server cluster.

```console
# HotStuff
> sudo apt-get install libssl-dev libuv1-dev cmake make
> cd $WORKDIR/cob/target_systems/hotstuff
> git submodule update --init --recursive
> ./build.sh
...
[ 83%] Built target hotstuff-app
[ 89%] Built target hotstuff-client
[ 94%] Built target hotstuff-keygen
[100%] Built target test_secp256k1

# Pompe and Pompe-SRO
> cd $WORKDIR/cob/target_systems/pompe
> git submodule update --init
> ./build.sh
...
[ 95%] Built target pompe-client
[ 95%] Built target hotstuff-app
[100%] Built target pompe-app

# Threshold VRF
> sudo apt-get install libprotobuf-dev protobuf-compiler pkg-config libsodium-dev libsodium23 libgmp-dev
> cd $WORKDIR/cob/target_systems/tvrf
> git submodule update --init --recursive
> mkdir build
> cd build
> cmake ..
> make -j
...
[100%] Built target generate_share
```

Make sure that you succeed in compiling HotStuff, Pompe, and TVRF before you proceed.
Both Pompe and Pompe-SRO are in the `target_systems/pompe` submodule, but in different git branches.

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
We map the 12 lines to **host0**..**host11** (i.e., **host0** represents Amsterdam and **host1** represents Austin, etc.).
You can setup latency emulation with COB:

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

The ping latency is 119*2=238ms because it is the round-trip latency between **host0** and **host1**.
Before running any experiment, make sure that latency emulation has been setup properly.

### Understand the Process of Running an Experiment

We now go through `config/pompe.toml`, and help you understand the process of running an experiment.

The `build = ["script/build_pompe_bias.sh"]` in `pompe.toml` means that COB first runs this script and builds the executable binaries for Pompe.

```shell
# build_pompe_bias.sh
cd target_systems/pompe; git checkout cob-pompe-lockstep; ./build.sh
```

The `cob-pompe-lockstep` branch of `target_systems/pompe` is prepared for measuing bias in Pompe.
With the binaries built, COB copies the config and binary files to all the 12 machines in `hostnames`.
The pathnames of the config/binary files are defined by the following lines of `pompe.toml`.

```console
config-files = ["conf-pompe-12"]
binary-files = ["pompe-client", "pompe-app"]
local-dir = ["/opt/home/cob/target_systems/pompe/examples/", "/opt/home/cob/target_systems/pompe/examples/"]
remote-dir = ["/opt/home/target_binary/", "/opt/home/target_config/"]
```

Specifically, for config files, COB will copy `/opt/home/cob/target_systems/pompe/examples/conf-pompe-12` from **control** to `/opt/home/target_config/` on **host0**..**host11**.
For binary files, COB will copy the two files `/opt/home/cob/target_systems/pompe/examples/pompe-client` and `/opt/home/cob/target_systems/pompe/examples/pompe-app` from **control** to directory `/opt/home/target_binary/` on **host0**..**host11**.
`pompe-client` is the client-side binary, and `pompe-app` is the server-side binary.
You don't need to modify anything here if you are reusing the CloudLab profiles we provide.

After copying all the files, COB runs the server binary on all the machines in `server-hosts`, and runs the client binary on all the machines in `client-hosts`.
By default, COB runs an experiment for 30 seconds, and then kills the client/server processes on all the 12 machines.
The stdout and stderr output from all the client/server processes will then be printed in the shell of **control** before COB terminates.
Other than stdout/stderr printing, the `/users/Yunhao/log/` directory (i.e., `log-dir` in `pompe.toml`) on each of **host0**..**host11** holds log files by the client/server processes.
These log files contain more details of the experiment result.
We will show you examples of how to read the printing and log files in the following detailed instructions.

## Detailed Instructions

We now explain how to run each of the 8 experiments.

### Run Experiment 1️⃣

Experiment #1 measures bias in HotStuff.
Read the following lines of `config/hotstuff.toml` first.
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
(i.e., 0.1ms is orders of magnitude lower than the emulated network latency).

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
In other words, it is calculated by the number of times the London client is ordered first divided by the total number of invocations.
Similarly, `Pr[W ≺ L]` is the sum of the second column divided by the sum of both columns.

The `-d 80000` above means that the experiment will last for 80000ms (i.e., 80 seconds).
Feel free to run the experiment longer, so you get a more accurate result.

### Run Experiment 2️⃣

Experiment #2 measures bias in Pompe.
Read the following lines of `config/pompe.toml` first.
The `client-hosts` shows that we are measuring bias between Munich and Tokyo this time.
Again, update `client-hosts` if you wish to measure bias between other cities.

```toml
# host3  London
# host4  Munich
# host9  Tokyo
# host11 Washington

# Pompe: 2 clients, measuring bias
client-hosts = ["host4", "host9"]
build = ["script/build_pompe_bias.sh"]
```

Note that there are 3 commented `client-hosts` and `build` in `config/pompe.toml`.
They will be used in later experiments, not Experiment #2.

#### Experiment #2 Command #1

This is the same as Experiment #1.

```console
> cd $WORKDIR/cob
> python3 script/sync_send.py
Server is listening on control:30000...
```

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

Without specifying `-d`, the experiment runs for 30 seconds by default.
Since we run the experiment for 30 seconds, and run consensus every 2 seconds in Pompe.
You can see about 15 `consensus ? finalized` in the printing above, each holding a batch of roughly 10 commands.
To get the result of Experiment #2, we need to inspect the log files from the two clients in `client-hosts`.
This is a chance of seeing the log files.

#### Get the Result of Experiment #2

```console
# client-hosts[0]
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

# client-hosts[1]
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
For each `idx`, the `median` of client1 is higher than that of client0, meaning that the Munich client (client0) is always ordered before the Tokyo client (client1),
leading to the number `1` in Figure 8.
You can ignore the `latency` column. It is just for debugging.

### Run Experiment 3️⃣

Experiment #3 measures bias in Themis.
If you are not using CloudLab, refer to the last section for installing the dependencies.

```console
> cd $WORKDIR/cob
> python3 script/themis_sim.py
Themis
Locatoin 1 < Location 2:  0
Location 2 < Location 1:  500
```

Note that `Location1` and `Location2` in [themis_sim.py](script/themis_sim.py) are `LONDON` and `WASHINGTON` respectively,
so the result above means that Themis is always biased towards the Washington client, leading to the number `1` in Figure 8.
You can change the `Location1` and `Location2` variables, run the Python script again, and obtain the other results for Themis in Figure 8.

The `num_txs=1000` in `themis_sim.py` means that we are simulating 1000 transactions (commands) in total.
This is why each of the 2 clients invokes 500 commands in this experiment, and the number printed above is 500.

### Run Experiment 4️⃣

Experiment #4 measures bias in Pompe-SRO.
Make sure to do the following for `build` in `config/pompe.toml`:

```toml
# host3  London
# host4  Munich
# host9  Tokyo
# host11 Washington

# Pompe-SRO: 2 clients, measuring bias
client-hosts = ["host4", "host9"]
build = ["script/build_pompe_sro_bias.sh"]
```

You need to run two commands in parallel on the **control** machine.

#### Experiment #4 Command #1

This is the same as Experiment #1 and #2.

```console
> cd $WORKDIR/cob
> python3 script/sync_send.py
Server is listening on control:30000...
```

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

Similar to Experiment #2, we need to inspect the log files on `host4` and `host9` for the experiment result.

#### Get the Result of Experiment #4

```console
# client-hosts[0]
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

# client-hosts[1]
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

### Run Experiment 5️⃣

Experiment #5 measures the effect of front-running in HotStuff, Pompe, Themis, and Pompe-SRO.

#### HotStuff, Pompe, and Pompe-SRO

Redo Experiments #1, #2, and #4 with 2 differences:

1. Run `script/frontrun.py` instead of `script/sync_send.py`.
2. Put the two clients on the same host in `client-hosts` of `pompe.toml`. For example, you can do `client-hosts = ["host6", "host6"]`.

`frontrun.py` allows the attacker to invoke its command 10ms earlier than the victim.
This does not make a big difference in Pompe-SRO, but HotStuff and Pompe will certainly order the attacker's command first.

#### Themis

Uncomment the following lines in `themis_sim.py`:

```python
ATTACKER = [25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25 ]
VICTIM   = [30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30 ]
Location1 = ATTACKER
Location2 = VICTIM
```

This allows the attacker to be 5ms faster than the victim to all the 12 cities, and Themis will be fully biased to the attacker just like HotStuff and Pompe.

### Run Experiment 6️⃣

Experiment #6 measures the performance of the two SRO implementations.
The SGX implementation of SRO is in [this repo](https://github.com/MaggieQi/ThemisTest/tree/main/sgx_sro), and please refer to its README about its usage.

The Threshold VRF implementation of SRO is in `cob/target_systems/tvrf`.

```console
> cd $WORKDIR/cob/target_systems/tvrf
> mkdir build
> cd build
> cmake ..
> make -j
...
[100%] Built target generate_share
> ./apps/random_beacon/generate_share
...
==========================
share generate latency (avg) = 316us
share combine latency        = 4913us
```

By default, the `generate_share` binary generates 100 shares, and [combines a random set of 68 shares](https://github.com/yhzhang0128/research-dvrf/blob/sro/apps/random_beacon/src/gen.cpp#L79-L84).
The printing above shows the average latency of generating the 100 shares, and the latency of combining 68 shares.
You can change the values of `nbNodes` and `threshold` in `apps/random_beacon/src/gen.cpp`.
Note that the algorithm combines `threshold+1` shares, so the current `threshold` is 67 instead of 68.

### Run Experiment 7️⃣

Experiment #7 measures the end-to-end performance of Pompe-SRO against Pompe.
Different from Experiment #2 and #4, Experiment #7 does NOT need `script/sync_send.py`.

#### Pompe Performance

Use the following lines of `config/pompe.toml`:

```toml
# Pompe: 12 clients, measuring performance
client-hosts = ["host0", "host1", "host2", "host3", "host4", "host5",
                "host6", "host7", "host8", "host9", "host10", "host11"]
build = ["script/build_pompe_perf.sh"]
```

Then run `cargo run eval -t pompe`. COB will spawn 12 clients this time:

```console
> cargo run eval -t pompe
...
[5/7] Spawn 12 server processes on remote hosts.
[6/7] Spawn 12 client processes on remote hosts.
  Terminate experiment after 30000ms.
[7/7] Kill 24 processes and collect output (may cause segfault during kill).
Killing client on host host3.
max_async_num = 200
[DEBUG] client3 receives 4875 ordering, 4512 consensus responses
[DEBUG] client3 ordering latency: median = 1.346353 sec, 90% = 1.459864 sec
[DEBUG] client3 consensus latency: median = 3.389179 sec, 90% = 4.056864 sec, 99% = 4.313350 sec
...
Killing client on host host2.
max_async_num = 200
[DEBUG] client2 receives 3839 ordering, 3512 consensus responses
[DEBUG] client2 ordering latency: median = 1.584098 sec, 90% = 1.786124 sec
[DEBUG] client2 consensus latency: median = 3.699636 sec, 90% = 4.471912 sec, 99% = 5.466683 sec
...
[DEBUG] server1 finished 59339 ordering phases; timer triggered 16 times; send 56643 exec responses to clients with 56643 callbacks
...
```

The output above only shows printings from client2, client3, and server1,
and printings from other clients and servers are omitted.
Since `host2` and `host3` represents Canberra and London,
the `client2 consensus latency` and `client3 consensus latency` above provide the **y-axis** latency statistics in Figure 12.
A convenient way of getting the throughput is to see the `server1 finished 59339 ordering phases` printing, and divide the number `59339` by 30 (i.e., the experiment period).
Therefore, the throughput is `59339/30=1978`, providing the **x-axis** statistics of the points.

Lastly, modifying `max_async_num` gives us different load from the 12 clients, leading to different system throughput.
For Figure 12, we run experiments with `max_async_num` being 10, 20, 40, 80, 120, 200, 400, and 600.
To update `max_async_num`, modify the `max-async` field of config file `cob/target_systems/pompe/examples/conf-pompe-12/hotstuff.gen.conf`.

#### Pompe-SRO Performance

Running Pompe-SRO is similar to running Pompe, and the only difference is in `config/pompe.toml`:

```toml
# Pompe-SRO: 12 clients, measuring performance
client-hosts = ["host0", "host1", "host2", "host3", "host4", "host5",
                "host6", "host7", "host8", "host9", "host10", "host11"]
build = ["script/build_pompe_sro_perf.sh"]
```

```console
> cargo run eval -t pompe
...
```
The way of obtaining the experiment result for Pompe-SRO is the same as Pompe.

### Run Experiment 8️⃣

Experiment #8 measures the performance of HotStuff.
Use the following lines of `config/hotstuff.toml`:

```toml
# Pompe: 2 clients, measuring performance
build = ["script/build_hotstuff_perf.sh"]
client-hosts = ["host3", "host2"]
```

Then run `cargo run eval -t hotstuff -d 60000`.
Again, `-d 60000` specifies the duration of the experiment.

```console
> cargo run eval -t hotstuff -d 60000
...
[5/7] Spawn 12 server processes on remote hosts.
[6/7] Spawn 2 client processes on remote hosts.
  Terminate experiment after 60000ms.
[7/7] Kill 14 processes and collect output (may cause segfault during kill).
...
client1 sent 103, executed 98 commands, max_async=5
[DEBUG] client1 consensus latency: median = 3.157715 sec, 90% = 3.158442 sec, 99% = 3.749544 sec
...
client0 sent 124, executed 119 commands, max_async=5
[DEBUG] client0 consensus latency: median = 2.583606 sec, 90% = 2.870634 sec, 99% = 2.871302 sec
...
```

In this experiment, `98+119=217` is the number of commands executed by HotStuff within 60 seconds,
so the throughput is `217/60=3.6` as shown in Figure 12 and 13.
The median and 99% percentile latencies are also shown in the printing above.
To obtain results with a higher load,
update `config/hotstuff.toml` and change the `"--max-async", "5"` to `"--max-async", "7"`.
HotStuff cannot handle more concurrent client commands because of its low throughput.

## Use a non-CloudLab Server Cluster

You can start by porting the latency emulation code to your cluster.
Specifically, update the `HOSTS` variable in `src/latency.rs`, and do `cargo run latency` on your control machine.
Again, use `ping` to test whether latency emulation has been setup properly.

### HotStuff, Pompe, Pompe-SRO

Revise all the entries in `hotstuff.toml` and `pompe.toml`, especially `local-dir` and `remote-dir`.
If the hostnames in your cluster are not **host0**..**host11**, you need to generate the encryption keys again. Specifically, you need to do 2 things.

1. In `target_systems/hotstuff` or `target_systems/pompe`, do `python3 scripts/gen_conf.py`. You need to modify the `ips` variable in `gen_conf.py` with the hostnames of your own servers.
2. Replace the crypto key files in `target_systems/hotstuff/examples/conf-pompe-12` and `target_systems/pompe/examples/conf-pompe-12`.

Again, remember to install the dependencies on all the machines.

```console
> sudo apt-get install libssl-dev libuv1-dev cmake make
```

Lastly, if you meet any problems, you could read and modify the Rust files under `src` of COB. There's not a lot of code.

### Themis

You need to install `numpy`, `cycler`, and `networkx` for Python3,
which are required by the `themis_protocol()` function from [the simulation code of Themis](https://github.com/anonthemis/themis-src-anon/blob/main/Aequitas-hotstuff/simulations/adv_reorder.py).

```console
> sudo apt update && sudo apt install python3-numpy python3-cycler python3-networkx
```

For any further questions, please contact Yunhao Zhang (yz2327@cornell.edu).
