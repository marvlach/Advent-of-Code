import os
import string
my_input_path = os.path.join(os.path.realpath(os.path.join(os.getcwd(), os.path.dirname(__file__))), 'data.txt')

def read_from_csv(my_input_path: str) -> list[list[int]]:

    rucksack = []
    
    with open(my_input_path) as file:
        lines = [line.rstrip() for line in file ]

    for line in lines:
        c = list(line)
        # d = { 'c1': c[:len(c)//2], 'c2': c[len(c)//2:]}
        rucksack.append(c)
    return rucksack

# given
racksack = read_from_csv(my_input_path)

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
    s1 += priority.index(common_item(r[:len(r)//2], r[len(r)//2:]).pop()) + 1
 
print(s1) #8109

# part 2
s2 = 0
for r in range(0, len(racksack), 3):
    s2 += priority.index(common_item(racksack[r],  racksack[r+1],  racksack[r+2]).pop()) + 1

print(s2) #2738