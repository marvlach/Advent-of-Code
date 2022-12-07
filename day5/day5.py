import os
import re

my_input_path = os.path.join(os.path.realpath(os.path.join(os.getcwd(), os.path.dirname(__file__))), 'data.txt')

def read_from_csv(my_input_path: str) -> tuple[dict[int, list[str]], list[tuple[int]]]:

    stack_dict_raw : dict[tuple, list[str]] = {}
    stack_dict : dict[int, list[str]] = {}
    instructions : list[tuple[int]]= []
    with open(my_input_path) as file:
        lines = [line.rstrip() for line in file ]
    reg = r"\[[A-Z]\]"
    for line in lines:
        
        for m in re.finditer(reg, line):
            if m.span() in stack_dict_raw:
                stack_dict_raw[m.span()].append(m.__getitem__(0))
            else:
                stack_dict_raw[m.span()] = [m.__getitem__(0)]

        line_split = line.split(' ')
        if line_split[0] == 'move':
            instructions.append(tuple(map(int, (line_split[1], line_split[3], line_split[5]))))
        

    stack_dict = {int((key[1] + 1)/4 ) : value[::-1] for key, value in stack_dict_raw.items()}
    stack_dict = dict(sorted(stack_dict.items()))
    return stack_dict, instructions

# given
stacks, instructions = read_from_csv(my_input_path)

# part 1
for num, fromStack, toStack in instructions:
    elements = stacks[fromStack][-num:]
    stacks[fromStack] = stacks[fromStack][:-num]
    stacks[toStack] = stacks[toStack] + elements[::-1]


print(''.join([list(v[-1])[1] for _, v in stacks.items()])) # SHMSDGZVC

# part 2
stacks, instructions = read_from_csv(my_input_path)

for num, fromStack, toStack in instructions:
    elements = stacks[fromStack][-num:]
    stacks[fromStack] = stacks[fromStack][:-num]
    stacks[toStack] = stacks[toStack] + elements


print(''.join([list(v[-1])[1] for _, v in stacks.items()])) # VRZGHDFBQ