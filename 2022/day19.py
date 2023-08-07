from input import input
from functools import cache


def read_input() -> (
    list[
        tuple[
            tuple[int, int, int],
            tuple[int, int, int],
            tuple[int, int, int],
            tuple[int, int, int],
        ],
    ]
):
    blueprints = list()

    for line in input():
        l = list()
        blueprint = line.split(":")[1].split(".")[:-1]
        l.append((int(blueprint[0].split()[-2]), 0, 0))
        l.append((int(blueprint[1].split()[-2]), 0, 0))
        so = blueprint[2].split()
        l.append((int(so[-5]), int(so[-2]), 0))
        sg = blueprint[3].split()
        l.append((int(sg[-5]), 0, int(sg[-2])))
        blueprints.append(tuple(l))

    return blueprints


blueprints = read_input()


def time_to_make_robot(
    resources_needed_for_target_robot: tuple[int, int, int],
    robots: tuple[int, int, int],
    resources: tuple[int, int, int],
) -> int | None:
    time_to_make_each_material_needed = []
    # loop through resources
    for total_resource_needed, current_resource, available_robots in zip(
        list(resources_needed_for_target_robot), list(resources), list(robots)
    ):
        more_resource_needed = total_resource_needed - current_resource

        # already have materials
        if more_resource_needed <= 0:
            time_to_make_each_material_needed.append(0)
            continue

        # if i need this material but don't have any robots, I cant make robot
        if available_robots == 0:
            return None

        time_to_make_each_material_needed.append(
            more_resource_needed // available_robots + (1 if more_resource_needed % available_robots else 0)
        )
    return max(time_to_make_each_material_needed)


def optimistic_more_achievable(current_robots: int, time_left: int) -> int:
    return current_robots * time_left + time_left * (time_left - 1) // 2


@cache
def max_value(
    running_max,
    blueprint: tuple[
        tuple[int, int, int],
        tuple[int, int, int],
        tuple[int, int, int],
        tuple[int, int, int],
    ],
    robots: tuple[int, int, int, int],
    resources: tuple[int, int, int, int],
    time_left: int,
) -> int:
    if time_left == 0:
        return max(running_max, resources[-1])

    if resources[-1] + optimistic_more_achievable(robots[-1], time_left) <= running_max:
        return running_max

    max_path = [0]
    new_running_max = running_max
    for robot_to_make in range(3, -1, -1):
        time_to_get_resources = time_to_make_robot(blueprint[robot_to_make], robots[:-1], resources[:-1])

        # if I can't build it or building it won't contribute any further
        if time_to_get_resources is None or time_to_get_resources >= time_left:
            # gather resources till the end
            new_resources = [res + rob * (time_left) for res, rob in zip(resources, robots)]
            new_time_left = 0
            max_path.append(max_value(new_running_max, blueprint, robots, tuple(new_resources), new_time_left))
            continue

        resources_after_get = [res + rob * time_to_get_resources for res, rob in zip(resources, robots)]

        # build
        resources_after_spent = [
            (r - blueprint[robot_to_make][i] if i < 3 else r) for i, r in enumerate(resources_after_get)
        ]

        # state after build
        new_resources = [res + rob for res, rob in zip(resources_after_spent, robots)]
        new_time_left = time_left - (time_to_get_resources + 1)
        new_robots = [r + 1 if i == robot_to_make else r for i, r in enumerate(robots)]

        new_running_max = max(
            new_running_max,
            max_value(new_running_max, blueprint, tuple(new_robots), tuple(new_resources), new_time_left),
        )
        max_path.append(new_running_max)

    return max(new_running_max, *max_path)


ql = []
for i, b in enumerate(blueprints):
    m = max_value(0, b, (1, 0, 0, 0), (0, 0, 0, 0), 24)
    print(m)
    ql.append((i + 1) * m)
    max_value.cache_clear()
print("part1", sum(ql))  # 1589

prod = 1
for i, b in enumerate(blueprints[:3]):
    m = max_value(0, b, (1, 0, 0, 0), (0, 0, 0, 0), 32)
    print(m)
    prod *= m
    max_value.cache_clear()
print("part2", prod)  # 29348 22, 29, 46
