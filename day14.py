from input import input
from copy import deepcopy


def read_input():

    tiles: set[tuple[int, int]] = set()
    max_y: int = 0
    for line in input():
        path = line.split(" -> ")
        for index in range(len(path) - 1):
            x1, y1 = [int(i) for i in path[index].split(",")]
            x2, y2 = [int(i) for i in path[index + 1].split(",")]
            tiles.add((x1, y1))
            tiles.add((x2, y2))
            if y1 >= max_y:
                max_y = y1
            if y2 >= max_y:
                max_y = y2

            for xx in range(min(x1, x2), max(x1, x2) + 1):
                for yy in range(min(y1, y2), max(y1, y2) + 1):
                    tiles.add((xx, yy))
                    if yy >= max_y:
                        max_y = yy

    return tiles, max_y


def part(tiles: set[tuple[int, int]], max_y: int, part: int):

    count_sand: int = 0
    sand = deepcopy(tiles)
    overflow: bool = False
    # new sand
    while True:
        sand_x = 500
        sand_y = 0

        # sand keeps falling
        while True:

            # overflow
            if part == 1 and sand_y > max_y:
                overflow = True
                break

            if part == 2 and sand_y == max_y - 1:
                break

            # sand can go down
            if (sand_x, sand_y + 1) not in sand:
                sand_y += 1
                continue

            # sand can go down-left
            if (sand_x - 1, sand_y + 1) not in sand:
                sand_y += 1
                sand_x -= 1
                continue

            # sand can go down-right
            if (sand_x + 1, sand_y + 1) not in sand:
                sand_y += 1
                sand_x += 1
                continue

            # sand at rest
            break

        # final sand rest
        if part == 2 and sand_y == 0 and sand_x == 500:
            return count_sand + 1

        if part == 1 and overflow:
            return count_sand

        count_sand += 1
        sand.add((sand_x, sand_y))


tiles, max_y = read_input()
print(part(tiles, max_y, 1))  # 1078
print(part(tiles, max_y + 2, 2))  # 30157
