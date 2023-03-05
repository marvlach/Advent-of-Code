from input import input


def read_input() -> list[list[int]]:

    my_input: list[list[int]] = []
    current_elf: list[int] = []

    for line in input():
        if line == "":
            my_input.append(current_elf)
            current_elf = []
        else:
            current_elf.append(int(line))
    my_input.append(current_elf)
    return my_input


def calories_per_elf(calories_per_item_per_elf: list[list[int]]) -> list[int]:
    return [sum(items_per_elf) for items_per_elf in calories_per_item_per_elf]


print(max(calories_per_elf(read_input())))  # 71934

print(sum(sorted(calories_per_elf(read_input()))[:3]))  # 26366
