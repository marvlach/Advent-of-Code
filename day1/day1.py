import os

my_input_path = os.path.join(os.path.realpath(os.path.join(os.getcwd(), os.path.dirname(__file__))), 'data.csv')

def read_from_csv(my_input_path: str) -> list[list[int]]:

    my_input : list[list[int]] = []
    current_elf : list[int] = []

    with open(my_input_path) as file:
        lines = [line.rstrip() for line in file ]

    for line in lines:
        if (line == ''):
            my_input.append(current_elf)
            current_elf = []
        else:
            current_elf.append(int(line))
    my_input.append(current_elf)
    return my_input


def calories_per_elf(calories_per_item_per_elf: list[list[int]]) -> list[int]:
    return [sum(items_per_elf) for items_per_elf in calories_per_item_per_elf]


print(max(calories_per_elf(read_from_csv(my_input_path))))

print(sum(sorted(calories_per_elf(read_from_csv(my_input_path)))[:3]))