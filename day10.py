from input import input
from enum import Enum


def read_input():

    running_sum = [1]

    for line in input():
        line_items = line.split()
        if len(line_items) == 1:
            running_sum.append(running_sum[-1])
        else:
            running_sum.append(running_sum[-1])
            running_sum.append(running_sum[-1] + int(line_items[1]))

    return running_sum


# part 1
skip = 20
limit = 40
final_cycle = 220
running_sum = read_input()

s = 0
for i in range(skip - 1, final_cycle, limit):
    s += (int(i) + 1) * running_sum[i]
print(s)  # 16020

# part 2
length = 40
height = 6
total_cycles = length * height
image = ["."] * length * height


def print_image(image: list[str], length: int, height: int):
    for r in list(range(height)):
        row = "".join(e for e in image[r * length : (r + 1) * length])
        print(row)


for cycle in range(total_cycles):
    currentl_drawing_horizontal = cycle % length
    if abs(running_sum[cycle] - currentl_drawing_horizontal) <= 1:
        image[cycle] = "#"

print_image(image, length, height)  # ECZUZALR
