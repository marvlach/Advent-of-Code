import os
import re

my_input_path = os.path.join(os.path.realpath(os.path.join(os.getcwd(), os.path.dirname(__file__))), 'data.txt')

def read_from_csv(my_input_path: str) -> list[str]:

    
    with open(my_input_path) as file:
        lines = [line.rstrip() for line in file ]
    return list(lines[0])

# given
characters = read_from_csv(my_input_path)

def find_start_of(l: list[str], h: int,) -> int:
    ret = 0
    for i, c in enumerate(l):
        if i < h - 1:
            continue
        if len(set(l[i-h+1:i+1])) == h:
            ret = i
            break
    return ret + 1

# part 1
print(find_start_of(characters, 4)) # 1210

# part 2
print(find_start_of(characters, 14)) # 3476


