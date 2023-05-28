import string
from input import input


def read_input() -> list[list[int]]:

    rucksack = []

    for line in input():
        c = list(line)
        rucksack.append(c)
    return rucksack


# given
racksack = read_input()

priority = list(string.ascii_lowercase + string.ascii_uppercase)


def common_item(*l: tuple[list]) -> set:
    ls: list[set] = []
    for i in l:
        ls.append(set(i))

    acc_set: set = ls[0]
    for s in ls:
        acc_set = acc_set.intersection(s)

    return acc_set


# part 1
s1 = 0
for r in racksack:
    s1 += priority.index(common_item(r[: len(r) // 2], r[len(r) // 2 :]).pop()) + 1

print(s1)  # 8109

# part 2
s2 = 0
for r in range(0, len(racksack), 3):
    s2 += (
        priority.index(common_item(racksack[r], racksack[r + 1], racksack[r + 2]).pop())
        + 1
    )

print(s2)  # 2738
