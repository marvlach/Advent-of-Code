from input import input
import re
import itertools
import functools


def read_input():
    valve = []
    rates = []
    targets = []
    for line in input():
        split_line = re.split(";|=|, ", line)
        valve.append(split_line[0].split()[1])
        rates.append(int(split_line[1]))
        first_valve = split_line[2].split()[-1]
        target = [first_valve]
        for i in range(3, len(split_line)):
            target.append(split_line[i])
        targets.append(target)
    return valve, rates, targets


valve, rates, targets = read_input()


def min_dist(valve: list[str], targets: list[list[str]]) -> list[list[str]]:
    G = [[1000] * len(valve) for _ in valve]
    for s, vs in enumerate(valve):
        for t, vt in enumerate(valve):
            if s == t:
                G[s][t] = 0
            elif vt in targets[s]:
                G[s][t] = 1

    dist = list(map(lambda p: list(map(lambda q: q, p)), G))

    for r, _ in enumerate(valve):
        for p, _ in enumerate(valve):
            for q, _ in enumerate(valve):
                dist[p][q] = min(dist[p][q], dist[p][r] + dist[r][q])
    return dist


dist = min_dist(valve, targets)
ivalve = {v: i for i, v in enumerate(valve)}


# F(s, t, S) = max{s'}[f(s') * (t - d(s, s') - 1) + F(s', t - d(s, s') - 1, S - {s'})]
@functools.cache
def max_value(t: int, s: str, S: set[str]):
    value_to_open_valve = [
        rates[ivalve[v]] * (t - dist[ivalve[s]][ivalve[v]] - 1)
        + max_value(t - dist[ivalve[s]][ivalve[v]] - 1, v, S - {v})
        for v in S
        if dist[ivalve[s]][ivalve[v]] < t
    ]
    return max(value_to_open_valve) if value_to_open_valve else 0


non_zero_flow_valves = {v for v in set(valve) if rates[ivalve[v]] > 0}

# part 1
print(max_value(30, "AA", frozenset(non_zero_flow_valves)))  # 2250


# part 2: assume all valves will be opened
def powerset(iterable):
    s = list(iterable)
    return itertools.chain.from_iterable(itertools.combinations(s, r) for r in range(len(s) + 1))


memo_set_values: dict[frozenset, int] = {}
for ps in powerset(non_zero_flow_valves):
    set_of_valves = frozenset(ps)
    memo_set_values[set_of_valves] = max_value(26, "AA", set_of_valves)

# player_1 opens k valves, achieving v == memo_set_values[k]
# player_2 opens the rest S - k valves, achieving memo_set_values[frozenset(non_zero_flow_valves - k)] independently
print(max([v + memo_set_values[frozenset(non_zero_flow_valves - k)] for k, v in memo_set_values.items()]))  # 3015
