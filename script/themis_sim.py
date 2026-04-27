# The themis_protocol() function is originally defined in this file:
# https://github.com/anonthemis/themis-src-anon/blob/main/Aequitas-hotstuff/simulations/adv_reorder.py

import numpy as np
from cycler import cycler
import networkx as nx
import copy
from itertools import combinations, permutations
import random

num_txs = 1000
n = 80        # number of nodes
f = 26

# Parameters
gen_dist = "exponential"
gen_param = 100
network_dist = "exponential"
network_param = 10000


def sample_nums(dist, param, num_vals):
    if dist == "uniform":
        return np.random.uniform(low=0, high=param*2, size=num_vals)
    elif dist == "exponential":
        return np.random.exponential(scale=param, size=num_vals)

AMSTERDAM     = [0,   119, 281, 9,   15,  36,  10,  142, 172, 270, 91,  81 ]
AUSTIN        = [119, 0,   190, 111, 126, 150, 114, 43,  274, 138, 41,  54 ]
CANBERRA      = [281, 190, 0,   275, 276, 296, 278, 156, 99,  221, 235, 206]
LONDON        = [9,   115, 275, 0,   20,  40,  9,   137, 172, 243, 86,  76 ]
MUNICH        = [15,  126, 275, 34,  0,   41,  16,  158, 178, 220, 109, 92 ]
OULU          = [36,  153, 288, 40,  41,  0,   46,  170, 187, 274, 121, 112]
PARIS         = [11,  114, 269, 8,   16,  45,  0,   146, 152, 272, 91,  78 ]
SANFRANCISCO  = [142, 42,  156, 137, 158, 170, 146, 0,   223, 107, 60,  71 ]
SINGAPORE     = [172, 274, 99,  172, 178, 187, 152, 223, 0,   83,  282, 270]
TOKYO         = [271, 139, 221, 243, 220, 274, 272, 107, 83,  0,   154, 163]
TORONTO       = [91,  41,  235, 87,  107, 121, 91,  62,  282, 154, 0,   71 ]
WASHINGTON    = [81,  54,  206, 76,  92,  111, 78,  71,  270, 163, 71,  0  ]

SERVERS = ["Amsterdam"] * 3 + ["Austin"] * 11 + ["Canberra"] * 3 + \
          ["London"] * 4 + ["Munich"] * 15 + ["Oulu"] * 6 + \
          ["Paris"] * 6 + ["SanFrancisco"] * 11 + ["Singapore"] * 4 + \
          ["Tokyo"] * 3 + ["Toronto"] * 3 + ["Washington"] * 11

Location1 = LONDON
Location2 = WASHINGTON

#Location1 = WASHINGTON
#Location2 = TOKYO

#Location1 = LONDON
#Location2 = MUNICH

#Location1 = MUNICH
#Location2 = TOKYO

#ATTACKER = [25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25 ]
#VICTIM   = [30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30 ]
#Location1 = ATTACKER
#Location2 = VICTIM

Latency_index = {"Amsterdam": 0, "Austin": 1, "Canberra": 2, "London": 3, "Munich": 4, "Oulu": 5, \
                 "Paris": 6, "SanFrancisco": 7, "Singapore": 8, "Tokyo": 9, "Toronto": 10, "Washington": 11}

def generate_all_timestamps():
    # Location1 and Location2 both invoke *half* transactions
    half = num_txs // 2

    # Generate Transaction Send timestamps
    send_time_diffs = sample_nums(gen_dist, gen_param, half)
    send_times = np.cumsum(send_time_diffs)
    send_times = np.sort(np.tile(send_times, 2))

    # Generate Transaction Network delays
    #network_delays = sample_nums(network_dist, network_param, (n,num_txs))
    network_delays = []
    for server in SERVERS:
        dst_index = Latency_index[server]
        network_delays.append([Location1[dst_index], Location2[dst_index]] * half)

    # Receive orderings for all nodes
    honest_node_orderings = send_times + network_delays

    assert(len(SERVERS)==n)
    return send_times, honest_node_orderings




def median_timestamp_protocol(node_orderings):
    medians = np.median(node_orderings[:n-f], axis=0)
    sort_indices = np.argsort(medians)
    # print(sort_indices)
    return sort_indices




def aequitas_protocol(gammas, node_orderings):
    graphs = []
    possible_pairs_all_gammas = []

    for gamma in gammas:
        G = nx.DiGraph()
        G.add_nodes_from(list(range(num_txs)))
        graphs.append(G)

        possible_pairs_all_gammas.append(set())


    # Reshape so now first index is tx instead of node
    np_node_orderings = np.transpose(node_orderings)

    # Pairs of txs
    pair_indices = np.array(list(permutations(range(np_node_orderings.shape[0]), 2)))
    
    # count number of times tx < tx'
    counts = np.count_nonzero((np_node_orderings[pair_indices[:,0],:] < np_node_orderings[pair_indices[:,1],:]), axis=1)

    for index in range(len(gammas)):
        gamma = gammas[index]
        order_indices = np.where(counts >= gamma * n - 2*f)

        for first,second in pair_indices[order_indices]:
            graphs[index].add_edge(*(first,second))


    orderings_per_gamma = []

    for gamma_index in range(len(gammas)):
        G = graphs[gamma_index]
        SCC = list(nx.strongly_connected_components(G))
        # print(SCC)
        H = nx.algorithms.components.condensation(G, SCC)

        ordering = nx.algorithms.dag.topological_sort(H)
        ordering_list = list(ordering)
        new_order_list = []
 
        for i in range(len(ordering_list)):
            ordering_list[i] = SCC[ordering_list[i]]
    
        for s in ordering_list:
            for val in sorted(s):
                new_order_list.append(val)

        orderings_per_gamma.append(new_order_list)

    # print(orderings_per_gamma)
    return orderings_per_gamma






def themis_protocol(gammas, node_orderings):
    graphs = []
    possible_pairs_all_gammas = []

    for gamma in gammas:
        G = nx.DiGraph()
        G.add_nodes_from(list(range(num_txs)))
        graphs.append(G)

        possible_pairs_all_gammas.append(set())


    # Reshape so now first index is tx instead of node
    np_node_orderings = np.transpose(node_orderings)

    # Pairs of txs
    pair_indices = np.array(list(permutations(range(np_node_orderings.shape[0]), 2)))
    
    # count number of times tx < tx'
    counts = np.count_nonzero((np_node_orderings[pair_indices[:,0],:] < np_node_orderings[pair_indices[:,1],:]), axis=1)

    for index in range(len(gammas)):
        gamma = gammas[index]
        threshold = n * (1-gamma) + f + 1
        total_counts = {}

        for i in range(len(pair_indices)):
            first, second = pair_indices[i]
            total_counts[(first,second)] = counts[i]
        
        for first, second in pair_indices:
            assert(total_counts[(first,second)] + total_counts[(second,first)] == n)
            if total_counts[(first,second)] >= threshold and total_counts[(second, first)] >= threshold:
                if total_counts[(first,second)] >= total_counts[(second,first)]:
                    graphs[index].add_edge(*(first,second))
                else:
                    graphs[index].add_edge(*(second,first))
            elif total_counts[(first,second)] >= threshold:
                graphs[index].add_edge(*(first,second))
            elif total_counts[(second,first)] >= threshold:
                graphs[index].add_edge(*(second,first))
            else:
                # This needs to be fixed since for small gamma, the bound on f is also smaller
                print("Error")
                print(threshold, total_counts[(first,second)], total_counts[(second,first)])


    orderings_per_gamma = []

    for gamma_index in range(len(gammas)):
        G = graphs[gamma_index]
        SCC = list(nx.strongly_connected_components(G))
        # print(SCC)
        H = nx.algorithms.components.condensation(G, SCC)

        ordering = nx.algorithms.dag.topological_sort(H)
        ordering_list = list(ordering)
        new_order_list = []
 
        for i in range(len(ordering_list)):
            ordering_list[i] = SCC[ordering_list[i]]
    
        for s in ordering_list:
            for val in sorted(s):
                new_order_list.append(val)

        orderings_per_gamma.append(new_order_list)

    # print(orderings_per_gamma)
    return orderings_per_gamma



def main():
    global gen_param
    global network_param

    num_pairs = int((num_txs * (num_txs - 1)) / 2)
    ratios = [1,10,100]

    labels = np.array(list(range(n,0, -2)))

    ratio = 25
    advs = [5,15,25]#[i for i in range(1,f+1)]
    network_param = gen_param * ratio

    #send_times, honest_node_orderings, bucket_counts, bucket_map = initialize()
    send_times, honest_node_orderings = generate_all_timestamps()
    print('Themis')
    gamma_honest_indices_themis = themis_protocol([1], honest_node_orderings)

    result = [0] * num_txs
    for i in range(num_txs):
        result[gamma_honest_indices_themis[0][i]] = i

    result_1_before_2 = result_2_before_1 = 0
    for i in range(num_txs // 2):
        location1_result = result[i * 2]
        location2_result = result[i * 2 + 1]
        if location1_result < location2_result:
            result_1_before_2 = result_1_before_2 + 1
        else:
            result_2_before_1 = result_2_before_1 + 1
    print("Locatoin 1 < Location 2: ", result_1_before_2)
    print("Location 2 < Location 1: ", result_2_before_1)

    medians = []
    themis_all = []


if __name__ == '__main__':
    main()
