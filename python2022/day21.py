from input import input


def read_input() -> list[int]:
    leafs = dict()
    for line in input():
        sp = line.split()
        # leaf
        if len(sp) == 2:
            leafs[sp[0][:-1]] = (sp[0][:-1], None, int(sp[1]), None, None)
        else:
            leafs[sp[0][:-1]] = (sp[0][:-1], "//" if sp[2] == "/" else sp[2], None, sp[1], sp[3])
    return leafs


D = read_input()


def calculate(D: dict, key: str):
    if D[key][2] is not None:
        return D[key][2]
    return eval(f"{calculate(D, D[key][3])} {D[key][1]} {calculate(D, D[key][4])}")


def where_is_human(D: dict, key: str, queue: list[int]) -> bool:
    # human
    if D[key][0] == "humn":
        return True
    # other leaf
    if D[key][2] is not None:
        return False

    try_left = where_is_human(D, D[key][3], queue)
    if try_left:
        queue.append("left")
        return try_left

    try_right = where_is_human(D, D[key][4], queue)
    if try_right:
        queue.append("right")
        return try_right
    return False


def get_new_target(target, operator, right, left):
    if operator == "+":
        return target - right if right else target - left
    if operator == "*":
        return target // right if right else target // left
    if operator == "-":
        return target + right if right else left - target
    if operator == "//":
        return target * right if right else left // target


def calculate2(D: dict, key: str, target: int, queue: list[int]) -> int:
    # human
    if D[key][0] == "humn":
        return target

    operator = D[key][1]
    left_or_right = queue.pop(-1)

    # variable on left, right is constant
    if left_or_right == "left":
        const_right = calculate(D, D[key][4])
        new_target = get_new_target(target, operator, const_right, None)
        return calculate2(D, D[key][3], new_target, queue)
    if left_or_right == "right":
        const_left = calculate(D, D[key][3])
        new_target = get_new_target(target, operator, None, const_left)
        return calculate2(D, D[key][4], new_target, queue)


print(calculate(D, "root"))  # 121868120894282

# change operation of root to be -
D["root"] = (D["root"][0], "-", None, D["root"][3], D["root"][4])

queue = []
where_is_human(D, "root", queue)

final = calculate2(D, "root", 0, queue)  # 3582317956029
print(final)
