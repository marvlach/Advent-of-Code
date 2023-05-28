from input import input
import re


def is_int(s):
    try:
        int(s)
        return True
    except ValueError:
        return False


def calculate_distance(source: tuple[int, int], target: tuple[int, int]):
    return abs(source[0] - target[0]) + abs(source[1] - target[1])


def read_input():

    SB: list = list()
    for line in input():
        split_line = re.split(",|=|:", line)
        nums = list()
        for str in split_line:
            if is_int(str):
                nums.append(int(str))
        SB.append(tuple(nums))

    dist = [calculate_distance((s_x, s_y), (b_x, b_y)) for s_x, s_y, b_x, b_y in SB]

    return SB, dist
   


def do_the_thing(row: int):

    ranges = []

    for i, (s_x, s_y, *_) in enumerate(SB):
        vertical_dist = abs(s_y - row)
        # it doesn't cut
        if vertical_dist > dist[i]:
            continue
        left = s_x - (dist[i] - vertical_dist)
        right = s_x + (dist[i] - vertical_dist)
        ranges.append([left, right])

    # sort margins asc by left
    sorted_ranges = sorted(ranges)

    intersected_ranges = [sorted_ranges[0]]
    current_range = 0
    for r in sorted_ranges[1:]:
        # if ranges overlap, extend current_range
        if r[0] <= intersected_ranges[current_range][1] + 1 and r[1] >= intersected_ranges[current_range][1]:
            intersected_ranges[current_range][1] = r[1]
            continue
        if r[0] > intersected_ranges[current_range][1] + 1:
            intersected_ranges.append(r)
            current_range += 1

    return intersected_ranges


SB, dist = read_input()


# part 1
intersected_ranges = do_the_thing(2_000_000)
count = 0
for not_intersected_range in intersected_ranges:
    count += not_intersected_range[1] - not_intersected_range[0]
print(count)  # 5127797

# part 2
beacon_row: int
beacon_col: int

for row in range(4_000_000):
    intersected_ranges = do_the_thing(row)
    if len(intersected_ranges) > 1:
        beacon_row, beacon_col = row, intersected_ranges[0][1] + 1
        break

print(4_000_000 * beacon_col + beacon_row)  # 12518502636475
