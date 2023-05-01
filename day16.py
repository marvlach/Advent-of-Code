from input import input
import re


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
def max_value(time_left: int, current: str, valves_left_to_open: set[str]):
    value_to_open_valve = [
        rates[ivalve[v]] * (time_left - dist[ivalve[current]][ivalve[v]] - 1)
        + max_value(time_left - dist[ivalve[current]][ivalve[v]] - 1, v, valves_left_to_open - {v})
        for v in valves_left_to_open
        if dist[ivalve[current]][ivalve[v]] < time_left
    ]
    return max(value_to_open_valve) if value_to_open_valve else 0


print(max_value(30, "AA", {v for v in set(valve) if rates[ivalve[v]] > 0}))  # 2250
