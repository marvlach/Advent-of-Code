import sys
from input import input
from math import gcd
from functools import cache


def read_input() -> list[tuple[int, int, str]]:
    blizzards = list()
    lines: list[str] = []
    for line in input():
        lines.append(line)

    rows = len(lines) - 2
    cols = len(lines[0]) - 2
    start = (-1, lines[0].index("."))
    end = (rows, lines[-1].index(".") - 1)
    for i, ll in enumerate(lines):
        for j, ch in enumerate(ll):
            if ch not in [".", "#"]:
                blizzards.append((i - 1, j - 1, ch))
    return start, end, blizzards, rows, cols


def get_snapshots(
    blizzards: list[tuple[int, int, str]],
    period: int,
    n: int,
    m: int,
) -> list[set[tuple[int, int]]]:
    snapshots = []
    for _ in range(period):
        snap_set = set()
        for i, (x, y, ch) in enumerate(blizzards):
            new_x = (x + 1) % n if ch == "v" else (x - 1) % n if ch == "^" else x
            new_y = (y + 1) % m if ch == ">" else (y - 1) % m if ch == "<" else y
            blizzards[i] = (new_x, new_y, ch)
            snap_set.add((new_x, new_y))
        snapshots.append(snap_set)
    last = snapshots.pop(-1)
    snapshots = [last, *snapshots[:]]
    return snapshots


def get_possible_to_go(
    pos: tuple[int, int],
    n_rows: int,
    n_cols: int,
    source: tuple[int, int],
    target: tuple[int, int],
    occupied: set[tuple[int, int]],
    forward: bool,
) -> list[tuple[int, int]]:
    x, y = pos
    dirs = [(0, 1), (1, 0), (-1, 0), (0, -1), (0, 0)] if forward else [(-1, 0), (0, -1), (0, 1), (1, 0), (0, 0)]
    new_positions = []
    for dx, dy in dirs:
        new_x, new_y = x + dx, y + dy
        if (new_x, new_y) not in occupied and (
            (0 <= new_x < n_rows and 0 <= new_y < n_cols) or (new_x, new_y) in [source, target]
        ):
            new_positions.append((new_x, new_y))
    return new_positions


@cache
def dfs(
    pos: tuple[int, int],
    t: int,
    n_rows: int,
    n_cols: int,
    source: tuple[int, int],
    target: tuple[int, int],
    period: int,
    snapshots: list[set[tuple[int, int]]],
    min_so_far: int,
    forward: bool,
) -> tuple[int, bool]:
    if pos == target:
        return t, True

    if t >= min_so_far:
        return t, False

    possible_to_go = get_possible_to_go(pos, n_rows, n_cols, source, target, snapshots[(t + 1) % period], forward)

    if len(possible_to_go) == 0:
        return period, False

    time_to_target = []
    found = []
    for p in possible_to_go:
        my_time, my_found = dfs(p, t + 1, n_rows, n_cols, source, target, period, snapshots, min_so_far, forward)
        time_to_target.append(my_time)
        found.append(my_found)

    min_so_far = min([min_so_far, *time_to_target])
    return min_so_far, any(found)


start, end, blizzards, rows, columns = read_input()
period = (rows * columns) // gcd(rows, columns)
snapshots = get_snapshots(blizzards, period, rows, columns)
freeze = tuple([frozenset(s) for s in snapshots])
sys.setrecursionlimit(2000)

time = 0
time += dfs(start, 0, rows, columns, start, end, period, freeze, period, True)[0]

print(time)  # 288
time += dfs(end, 0, rows, columns, end, start, period, freeze, period, False)[0]
time += dfs(start, 0, rows, columns, start, end, period, freeze, period, True)[0]
print(time)  # 861
