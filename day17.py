from input import input
import collections
import itertools


def read_input():
    direction = []

    for line in input():
        direction = list(line)

    return direction


def get_next_direction(dir):
    i = 0
    while True:
        yield i, dir[i]
        i = (i + 1) % len(dir)


def get_next_shape():
    shapes = [
        [(0, 2), (0, 3), (0, 4), (0, 5)],
        [(2, 3), (1, 2), (1, 3), (1, 4), (0, 3)],
        [(2, 4), (1, 4), (0, 4), (0, 2), (0, 3)],
        [(3, 2), (2, 2), (1, 2), (0, 2)],
        [(1, 2), (1, 3), (0, 2), (0, 3)],
    ]
    i = 0
    while True:
        yield i, shapes[i]
        i = (i + 1) % len(shapes)


def get_new_position(current, dx, dy, S, limit_y=7):
    potential_new_position = [(point_x + dx, point_y + dy) for (point_x, point_y) in current]

    if any([(point_x, point_y) in S for (point_x, point_y) in potential_new_position]):
        return current, True if dy == 0 else False

    if any([point_y > limit_y - 1 or point_y < 0 for (_, point_y) in potential_new_position]):
        return current, False

    return potential_new_position, False


def do_the_thing(N, n_cols):
    top_height = 0
    height_reached = []
    num_of_rocks = 0

    set_of_covered = set([(0, y) for y in range(n_cols)])
    max_of_every_column_normalized = [0] * n_cols

    # assume cycle when tuple (rock_id, dir_id, tuple(max_of_every_column_normalized)) has been seen
    combos_seen = collections.OrderedDict()

    generate_rocks = get_next_shape()
    generate_directions = get_next_direction(directions)

    while num_of_rocks < N:
        # rocks spawns with its lowest = top_height + 4
        rock_id, rock = next(generate_rocks)
        rock_position = [(point_x + top_height + 4, point_y) for point_x, point_y in rock]

        while True:
            dir_id, dir = next(generate_directions)
            direction = +1 if dir == ">" else -1

            if (rock_id, dir_id, tuple(max_of_every_column_normalized)) in combos_seen:
                return True, height_reached, combos_seen[(rock_id, dir_id, tuple(max_of_every_column_normalized))]

            combos_seen[(rock_id, dir_id, tuple(max_of_every_column_normalized))] = num_of_rocks
            rock_position, _ = get_new_position(rock_position, 0, direction, set_of_covered)
            rock_position, rock_is_rest = get_new_position(rock_position, -1, 0, set_of_covered)
            if rock_is_rest:
                break
        set_of_covered |= set(rock_position)
        num_of_rocks += 1
        top_height = max([point_x for point_x, _ in set_of_covered])
        height_reached.append(top_height)

        for x, y in set_of_covered:
            if x > max_of_every_column_normalized[y]:
                max_of_every_column_normalized[y] = x

        offset = min(max_of_every_column_normalized)
        max_of_every_column_normalized = [m - offset for m in max_of_every_column_normalized]

    return False, height_reached, -1


directions = read_input()
N = 1_000_000_000_000  # 1537175792495
# N = 2022 # 3106

pattern_found, height_reached, pattern_start = do_the_thing(2022, 7)
if pattern_found:
    offset = height_reached[:pattern_start]
    cycle = height_reached[pattern_start:]
    offset_height = offset[-1]
    offset_rocks = len(offset)

    cycle_height = cycle[-1] - offset[-1]
    cycle_rocks = len(cycle)

    num_of_cycles = (N - offset_rocks) // cycle_rocks
    rocks_of_last_cycle = (N - offset_rocks) % cycle_rocks
    print(offset_rocks, cycle_rocks, offset_height, cycle_height, rocks_of_last_cycle)
    print(pattern_found, num_of_cycles * cycle_height + offset_height + (cycle[rocks_of_last_cycle] - offset[-1]))


else:
    print(height_reached[-1])
