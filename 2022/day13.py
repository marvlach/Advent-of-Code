from input import input
from copy import deepcopy
from functools import cmp_to_key


def fill_list(list_to_fill: list, line_list: list):
    if not line_list:
        return
    # get character
    char: str = line_list.pop(0)

    if char == "]":
        return

    if char == "[":
        nested_list = []
        fill_list(nested_list, line_list)
        list_to_fill.append(nested_list)

    if char.isnumeric():
        # special number 10
        if int(char) == 1 and line_list[0].isnumeric() and int(line_list[0]) == 0:
            char = "".join([char, line_list[0]])
            line_list.pop(0)
        list_to_fill.append(int(char))

    fill_list(list_to_fill, line_list)


def read_input():

    packets = []
    packet: int = 0

    for line in input():
        if line == "":
            packet = 0
            continue

        line_list = list(line)
        if packet == 0:
            left = []
            fill_list(left, line_list)
            packets.append(left[0])
        else:
            right = []
            fill_list(right, line_list)
            packets.append(right[0])

        packet += 1

    return packets


def compare(left_arg: list | int, right_arg: list | int):
    left = deepcopy(left_arg)
    right = deepcopy(right_arg)
    left_is_list = isinstance(left, list)
    right_is_list = isinstance(right, list)

    # both int
    if not (left_is_list or right_is_list):
        if left < right:
            return 1

        if left > right:
            return -1

        return 0

    # mixed types
    if left_is_list and not right_is_list:
        return compare(left, [right])

    if right_is_list and not left_is_list:
        return compare([left], right)

    # both lists
    left_is_empty = len(left) == 0
    right_is_empty = len(right) == 0

    if left_is_empty and not right_is_empty:
        return 1

    if right_is_empty and not left_is_empty:
        return -1

    if left_is_empty and right_is_empty:
        return 0

    # both not empty
    left_item = left.pop(0)
    right_item = right.pop(0)

    result = compare(left_item, right_item)
    if result != 0:
        return result

    return compare(left, right)


# part 1
packets = read_input()
indexes = []
for i in range(0, len(packets), 2):
    left, right = packets[i], packets[i + 1]
    if compare(left, right) == 1:
        indexes.append(i // 2 + 1)
print(sum(indexes))  # 6369

# part 2
sep1 = [[2]]
sep2 = [[6]]
packets.append(sep1)
packets.append(sep2)
sorted_packets = sorted(packets, reverse=True, key=cmp_to_key(compare))
print((sorted_packets.index(sep1) + 1) * (sorted_packets.index(sep2) + 1))  # 25800
