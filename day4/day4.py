import os
import re

my_input_path = os.path.join(os.path.realpath(os.path.join(os.getcwd(), os.path.dirname(__file__))), 'data.txt')

def read_from_csv(my_input_path: str) -> list[list[int]]:

    sections: list[list[int]] = []
    
    with open(my_input_path) as file:
        lines = [line.rstrip() for line in file ]

    for line in lines:
        ls = re.split('-|,', line)
        li = [int(i) for i in ls]
        sections.append(li)
    return sections

# given
sections = read_from_csv(my_input_path)

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
