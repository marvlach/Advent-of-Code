from input import input
from math import gcd
from functools import partial
from typing import Callable


def find_first(in_list: list[str], char: str) -> int:
    return in_list.index(char) if char in in_list else len(in_list) + 1


def find_last(in_list: list[str], char: str) -> int:
    return len(in_list) - list(reversed(in_list)).index(char) - 1 if char in in_list else -1


def transposed(M: list[list[str]]) -> list[list[str]]:
    return list(map(list, zip(*M)))


def read_input() -> tuple[list[list[str]], list[int | str], int]:
    M = []
    for line in input():
        tiles = list(line)
        M.append(tiles)

    last_line = M.pop(-1)
    _ = M.pop(-1)

    max_col = max([len(row) for row in M])

    acc = ""
    D = []
    for d in last_line:
        if d in ["R", "L"]:
            D.append(int(acc))
            acc = ""
            D.append(d)
        else:
            acc += d
    D.append(int(acc))
    return [row + (max_col - len(row)) * [" "] for row in M], D, max_col


M, D, max_col = read_input()
max_row = len(M)
assert all([len(row) == max_col for row in M])

range_row = [
    (
        min(find_first(row, "."), find_first(row, "#"), len(row)),
        max(find_last(row, "."), find_last(row, "#"), 0),
    )
    for row in M
]

range_col = [
    (
        min(find_first(row, "."), find_first(row, "#"), len(row)),
        max(find_last(row, "."), find_last(row, "#"), 0),
    )
    for row in transposed(M)
]


def rotate(current: str, rot: str) -> str:
    directions = ["R", "D", "L", "U"]
    return directions[(directions.index(current) + (1 if rot == "R" else -1)) % len(directions)]


def step(
    M: list[list[str]],
    state: tuple[int, int, str],
    range_row: list[tuple[int, int]],
    range_col: list[tuple[int, int]],
    out_of_bounds_handler: Callable,
) -> tuple[int, int, str]:
    x, y, d = state

    # dump step
    next_x = x + 1 if d == "D" else x - 1 if d == "U" else x
    next_y = y + 1 if d == "R" else y - 1 if d == "L" else y
    next_d = d

    if (d in ["R", "L"] and not (range_row[next_x][0] <= next_y <= range_row[next_x][1])) or (
        d in ["D", "U"] and not (range_col[next_y][0] <= next_x <= range_col[next_y][1])
    ):
        next_x, next_y, next_d = out_of_bounds_handler(state)

    # wall
    if M[next_x][next_y] == "#":
        return (x, y, d)

    return (next_x, next_y, next_d)


def run(
    M: list[list[str]],
    D: list[int | str],
    range_row: list[tuple[int, int]],
    range_col: list[tuple[int, int]],
    initial: tuple[int, int, str],
    out_of_bound_handler: Callable,
):
    state = initial
    for a in D:
        x, y, d = state
        # rotate
        if a in ["R", "L"]:
            state = (x, y, rotate(d, a))
            continue
        # take steps
        for _ in range(a):
            new_state = step(M, state, range_row, range_col, out_of_bound_handler)
            # wall
            if new_state == state:
                break
            state = new_state

    return state


def out_of_bounds_handler_part_1(
    state: tuple[int, int, str],
    range_row: list[tuple[int, int]],
    range_col: list[tuple[int, int]],
) -> tuple[int, int, str]:
    next_x, next_y, d = state

    # i know i am out
    if d == "R":
        next_y = range_row[next_x][0]
    elif d == "L":
        next_y = range_row[next_x][1]
    elif d == "D":
        next_x = range_col[next_y][0]
    elif d == "U":
        next_x = range_col[next_y][1]

    return next_x, next_y, d


f1_x, f1_y, f1_d = run(
    M,
    D,
    range_row,
    range_col,
    (0, range_row[0][0], "R"),
    partial(out_of_bounds_handler_part_1, range_row=range_row, range_col=range_col),
)
print(1000 * (f1_x + 1) + 4 * (f1_y + 1) + ["R", "D", "L", "U"].index(f1_d))  # 117102


# part 2
def get_patches_top_left(M: list[list[str]], max_row: int, max_col: int, S: int) -> list[tuple[int, int]]:
    patches = list()
    for row in range(0, max_row, S):
        for col in range(0, max_col, S):
            if M[row][col] != " ":
                patches.append((row, col))
    return patches


S = gcd(max_row, max_col)
patches = get_patches_top_left(M, max_row, max_col, S)


# cube graph
def get_immediate_neighbours(patches: list[tuple[int, int]], patch: tuple[int, int], S: int):
    x, y = patch
    neighbours = {}
    N = [(x, y - S), (x, y + S), (x - S, y), (x + S, y)]
    dir = ["L", "R", "U", "D"]
    for p, d in zip(N, dir):
        if p in patches:
            neighbours[p] = d
    return neighbours


# keys: patch top left, values: dicts with keys patch top left , values: LRUD
N = {p: get_immediate_neighbours(patches, p, S) for p in patches}

# any know cube will do, keys are (side, num of clockwise rotations)
c_cube = {
    1: {"L": (5, 3), "R": (2, 0), "D": (3, 1), "U": (6, 0)},
    2: {"L": (1, 0), "R": (4, 3), "D": (3, 0), "U": (6, 1)},
    3: {"L": (1, 3), "R": (4, 0), "D": (5, 1), "U": (2, 0)},
    4: {"L": (3, 0), "R": (6, 3), "D": (5, 0), "U": (2, 1)},
    5: {"L": (3, 3), "R": (6, 0), "D": (1, 1), "U": (4, 0)},
    6: {"L": (5, 0), "R": (2, 3), "D": (1, 0), "U": (4, 1)},
}


def account_for_rotation(dir, rot):
    rot_map = [
        {"L": "L", "R": "R", "U": "U", "D": "D"},
        {"L": "D", "R": "U", "U": "L", "D": "R"},
        {"L": "R", "R": "L", "U": "D", "D": "U"},
        {"L": "U", "R": "D", "U": "R", "D": "L"},
    ]
    return rot_map[rot][dir]


def bfs(patches, N, cube):
    queue = [(patches[0], (1, 0))]
    visited = set()
    visited.add(patches[0])
    mapping = {}
    while queue:
        cur, (side, rot) = queue.pop(0)
        mapping[cur] = (side, rot)
        for patch, dir in N[cur].items():
            if patch not in visited:
                patch_side, additional_rotations = cube[side][account_for_rotation(dir, rot)]
                visited.add(patch)
                queue.append((patch, (patch_side, (rot + additional_rotations) % 4)))
    return mapping


def map_point_rotate(point: tuple[int, int], num_of_rots: int, S: int) -> tuple[int, int]:
    x, y = point
    for _ in range(num_of_rots):
        x, y = y, S - x - 1
    return (x, y)


def get_patch_from_point(point, patches):
    x, y = point
    for p in patches:
        if p[0] <= x < p[0] + S and p[1] <= y < p[1] + S:
            return p


mapping = bfs(patches, N, c_cube)


def out_of_bounds_handler_part_2(state: tuple[int, int, str], patches, mapping, cube):
    x, y, d = state

    # source state in global coords to source state in patch coords
    source_patch_offset = get_patch_from_point((x, y), patches)
    known_source_patch, known_source_patch_rotation = mapping[source_patch_offset]
    r_x, r_y = map_point_rotate(
        (x - source_patch_offset[0], y - source_patch_offset[1]), 4 - known_source_patch_rotation, S
    )
    r_d = account_for_rotation(d, known_source_patch_rotation)

    # get new patch and new adjusted direction
    target_patch, target_patch_relative_rotation_to_source = cube[known_source_patch][r_d]

    # step
    if r_d == "U":
        unadjusted_x, unadjusted_y = S - 1, r_y
    elif r_d == "D":
        unadjusted_x, unadjusted_y = 0, r_y
    elif r_d == "R":
        unadjusted_x, unadjusted_y = r_x, 0
    elif r_d == "L":
        unadjusted_x, unadjusted_y = r_x, S - 1

    # adjust state based on known rotation between source and target
    adjusted_x, adjusted_y = map_point_rotate(
        (unadjusted_x, unadjusted_y), 4 - target_patch_relative_rotation_to_source, S
    )
    adjusted_d = account_for_rotation(r_d, target_patch_relative_rotation_to_source)

    # target state in known cube to global state
    (target_offset_x, target_offset_y), new_rot = [
        (point, side_rot) for point, (side, side_rot) in mapping.items() if side == target_patch
    ][0]
    target_relative_x, target_relative_y = map_point_rotate((adjusted_x, adjusted_y), new_rot, S)
    global_x, global_y = target_offset_x + target_relative_x, target_offset_y + target_relative_y
    global_d = account_for_rotation(adjusted_d, (4 - new_rot) % 4)

    return global_x, global_y, global_d


f2_x, f2_y, f2_d = run(
    M,
    D,
    range_row,
    range_col,
    (0, range_row[0][0], "R"),
    partial(out_of_bounds_handler_part_2, patches=patches, mapping=mapping, cube=c_cube),
)
print(1000 * (f2_x + 1) + 4 * (f2_y + 1) + ["R", "D", "L", "U"].index(f2_d))  # 135297
