from input import input
from copy import deepcopy


def read_input() -> list[tuple[int, int]]:
    elf_positions = list()
    for elf_x, line in enumerate(input()):
        elfs_in_line = [index for index, val in enumerate(line) if val == "#"]
        for elf_y in elfs_in_line:
            elf_positions.append((elf_x, elf_y))

    return elf_positions


def get_neigh_positions(x: int, y: int) -> list[tuple[int, int]]:
    diffs = [
        (-1, -1, "NW"),
        (-1, 0, "N"),
        (-1, 1, "NE"),
        (0, -1, "W"),
        (0, 1, "E"),
        (1, -1, "SW"),
        (1, 0, "S"),
        (1, 1, "SE"),
    ]
    return [(x + dx, y + dy, r) for (dx, dy, r) in diffs]


def propose(
    elf: int,
    elf_positions: list[tuple[int, int]],
    mut_orders: list[list[int]],
) -> tuple[int, int]:
    elf_x, elf_y = elf_positions[elf]
    free = [True] * 8
    general = [[0, 1, 2], [5, 6, 7], [0, 3, 5], [2, 4, 7]]
    for i, (n_x, n_y, r) in enumerate(get_neigh_positions(elf_x, elf_y)):
        if (n_x, n_y) in elf_positions:
            free[i] = False

    if all(free):
        return elf_x, elf_y

    propose, found_free = 0, False
    for i, dir in enumerate(mut_orders):
        # [0, 1, 2] | [5, 6, 7] | [1, 3, 5] | [2, 4, 7]
        directions = general[dir]

        if all([free[d] for d in directions]):
            propose, found_free = dir, True
            break
    if not found_free:
        return elf_x, elf_y

    diffs = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    return elf_x + diffs[propose][0], elf_y + diffs[propose][1]


def round(mut_elf_positions: list[tuple[int, int]], mut_orders: list[int]):
    elf_props = list()
    pos: dict[tuple[int, int], list[int]] = dict()
    # propose
    for el, _ in enumerate(mut_elf_positions):
        elf_propose = propose(el, mut_elf_positions, mut_orders)
        elf_props.append(elf_propose)
        if elf_propose not in pos:
            pos[elf_propose] = [el]
        else:
            pos[elf_propose].append(el)

    # move
    for el, _ in enumerate(mut_elf_positions):
        if len(pos[elf_props[el]]) == 1:
            mut_elf_positions[el] = elf_props[el]
    # step 3
    first = mut_orders.pop(0)
    mut_orders.append(first)


def part_1(elves, orders):
    elves_part = deepcopy(elves)
    orders_part = deepcopy(orders)

    for r in range(10):
        round(elves_part, orders_part)

    min_x = min([x for x, y in elves_part])
    max_x = max([x for x, y in elves_part])
    min_y = min([y for x, y in elves_part])
    max_y = max([y for x, y in elves_part])
    return (max_x - min_x + 1) * (max_y - min_y + 1) - len(elves_part)


def part_2(elves, orders):
    elves_part = deepcopy(elves)
    orders_part = deepcopy(orders)
    prev = set(elves_part)
    count = 0
    while True:
        round(elves_part, orders_part)
        if prev == set(elves_part):
            break
        prev = set(elves_part)
        count += 1
    return count + 1


elves = read_input()
orders = list(range(4))

print(part_1(elves, orders))  # 4254
print(part_2(elves, orders))  # 992
