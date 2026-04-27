# Benchmark for Equal Opportunity in Ordered Consensus

This benchmark toolkit named Chance Oracle Benchmark (COB) is developed for paper [**Equal Opportunity: A Correctness Condition for Ordered Consensus**](https://www.usenix.org/conference/osdi26/presentation/zhang-yunhao) at OSDI'26.

## Claims, Figures, and Experiments

There are 8 experiments leading to the 5 figures in the evaluation section of this paper.
The table below summarizes the claims supported by these figures and experiments.

| Figures | Experiments    | Claims                                                                                                 |
|---------|----------------|--------------------------------------------------------------------------------------------------------|
| Fig. 8  | Exp #1, #2, #3 | The 3 baseline systems are all vulnerable to bias.                                                     |
| Fig. 9  | Exp #4         | Pompe-SRO can control the degree of bias under a target Ɛ.                                             |
| Fig. 10 | Exp #5         | Attacker gets the mean reward in Pompe-SRO and the max in baselines                                    |
| Fig. 11 | Exp #6         | SRO latency is low and SRO is not the performance bottleneck in Pompe-SRO.                             |
| Fig. 12 | Exp #7, #8     | Pompe-SRO maintains the same end-to-end throughput as Pompe and incurs a latency overhead of 47%-67%.  |

The sections below provide the details of the 8 experiments.
