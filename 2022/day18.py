from input import input


def read_input() -> tuple[set[tuple[int, int, int]], tuple[tuple[int, int], tuple[int, int], tuple[int, int]]]:
    cubes = set()

    for line in input():
        cubes.add(tuple([int(d) for d in line.split(",")]))
    margins = [[cube[dim] for cube in cubes] for dim in range(3)]
    ranges = [(min(dim_values) - 1, max(dim_values) + 1) for dim_values in margins]
    return cubes, tuple(ranges)


cubes, ranges = read_input()
print(ranges)


def get_new_cube_by_displacement_in_dimension(cube, dim, df):
    new_cube = [d for d in cube]
    new_cube[dim] += df
    return tuple(new_cube)


def get_neighbours(cube: tuple[int, int, int]) -> list[tuple[int, int, int]]:
    neighbours = []
    for dim in range(3):
        for diff in [-1, 1]:
            neighbours.append(get_new_cube_by_displacement_in_dimension(cube, dim, diff))
    return neighbours


def count_sides(cubes: set[tuple[int, int, int]], part: int, outside: set[tuple[int, int, int]]):
    count = 0
    for cube in cubes:
        for neighbouring_cube in get_neighbours(cube):
            if (neighbouring_cube not in cubes) and (part == 1 or neighbouring_cube in outside):
                count += 1
    return count


def is_in_space(cube, ranges):
    return all([dim_range[0] <= cube[dim_i] <= dim_range[1] for dim_i, dim_range in enumerate(ranges)])


def outside_air_cubes(cubes: set[tuple[int, int, int]], ranges):
    queue = [(ranges[0][0], ranges[1][0], ranges[2][0])]
    visited = set((ranges[0][0], ranges[1][0], ranges[2][0]))
    while queue:
        point = queue.pop(0)
        neighbours = get_neighbours(point)
        for neighbour in neighbours:
            if not is_in_space(neighbour, ranges) or neighbour in cubes:
                continue

            if neighbour not in visited:
                queue.append(neighbour)
                visited.add(neighbour)

    return visited


print(count_sides(cubes, 1, []))  # 3550
print(count_sides(cubes, 2, outside_air_cubes(cubes, ranges)))  # 2028
