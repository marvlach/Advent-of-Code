from input import input


def read_input() -> list[str]:

    for line in input():
        pass
    return list(line)


# given
characters = read_input()


def find_start_of(
    l: list[str],
    h: int,
) -> int:
    ret = 0
    for i, c in enumerate(l):
        if i < h - 1:
            continue
        if len(set(l[i - h + 1 : i + 1])) == h:
            ret = i
            break
    return ret + 1


# part 1
print(find_start_of(characters, 4))  # 1210

# part 2
print(find_start_of(characters, 14))  # 3476
