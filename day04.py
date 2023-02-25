import re
from input import input


def read_input() -> list[list[int]]:

    sections: list[list[int]] = []
    
    for line in input():
        ls = re.split('-|,', line)
        li = [int(i) for i in ls]
        sections.append(li)
    return sections

# given
sections = read_input()

# part 1
def full_overlap(l: list[int]) -> bool: # [start0, end0, start1, end1]
    return True if (l[2] <= l[0] and l[3] >= l[1]) or (l[2] >= l[0] and l[3] <= l[1]) else False

n = 0
for pair in sections:
    n = n + 1 if full_overlap(pair) else n
print(n) # 573

# part 2
def no_overlap(l: list[int]) -> bool: #[start0, end0, start1, end1]
    return True if (l[3] < l[0] or l[2] > l[1]) else False

n = 0
for pair in sections:
    n = n if no_overlap(pair) else n + 1
print(n) # 867
