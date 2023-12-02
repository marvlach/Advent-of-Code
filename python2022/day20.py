from input import input


def read_input() -> list[int]:
    init_list = list()

    for line in input():
        init_list.append(int(line))
    return init_list


read_list = read_input()


def rearrange(mutable: dict[tuple[int, int], int], position: int, new_position: int):
    for (affected_id, affected_value), current_index in mutable.items():
        if min(position, new_position) <= current_index <= max(position, new_position):
            mutable[(affected_id, affected_value)] += 1 if new_position < position else -1


def mix(copy_original: list[int], mutable: dict[tuple[int, int], int]):
    for id, v in enumerate(copy_original):
        position, offset = mutable[(id, v)], v
        new_position = (position + offset) % (len(copy_original) - 1)
        rearrange(mutable, position, new_position)
        mutable[(id, v)] = new_position

    return mutable


def run(original: list[int], part: 1 | 2):
    copy_original = [(1 if part == 1 else 811589153) * item for item in original]
    mutable = {(id, v): id for id, v in enumerate(copy_original)}
    for _ in range(10 if part == 2 else 1):
        mix(copy_original, mutable)
    s = 0
    for i in [1000, 2000, 3000]:
        the_index = (i + mutable[(read_list.index(0), 0)]) % len(read_list)
        get_value = [value for (_, value), final_index in mutable.items() if final_index == the_index][0]
        s += get_value
    return s


print(run(read_list, 1))  # 13967
print(run(read_list, 2))  # 1790365671518
