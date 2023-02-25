
def input():
    with open('data.txt') as file:
        for line in file:
            yield line.rstrip()
