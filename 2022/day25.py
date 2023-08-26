import sys
from input import input
from math import gcd
from functools import cache


def read_input() -> list[str]:
    nums = []
    for line in input():
        nums.append(line)
    return nums


def to_decimal(num: str) -> int:
    p = 0
    s = 0
    lex = {"0": 0, "1": 1, "2": 2, "-": -1, "=": -2}
    for digit in reversed(num):
        s += lex[digit] * (5**p)
        p += 1
    return s


def to_base_5(num: int) -> list[int]:
    if num == 0:
        return []
    return [num % 5, *to_base_5(num // 5)]


def to_snafu(num: int) -> str:
    base_5 = to_base_5(num)
    final, rem = list(), 0
    while base_5 or rem != 0:
        digit = (base_5.pop(0) if base_5 else 0) + rem
        rem = 0 if digit < 3 else 1
        lex = [0, 1, 2, '=', '-', 0]
        final.append(str(lex[digit]))
    return "".join(list(reversed(final)))


def part_1(nums: list[str]):
    s = 0
    for num in nums:
        s += to_decimal(num)
    return to_snafu(s)


nums = read_input()
print(part_1(nums)) # 2-2=12=1-=-1=000=222
