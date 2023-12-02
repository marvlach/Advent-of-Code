from input import input


def read_input():

    monkeys: list[list[int]] = []
    operations: list[str] = []
    operands: list[int] = []
    test: list[int] = []
    true: list[int] = []
    false: list[int] = []

    for line in input():
        if line == "":
            monkeys.append(items)
            operations.append(operation)
            operands.append(operand)
            test.append(test_operand)
            true.append(true_target_index)
            false.append(false_target_index)
            continue

        line_items = line.split()

        if line_items[0] == "Monkey":
            continue
        elif line_items[0] == "Starting":
            items = [int(item.rstrip(",")) for item in line_items[2:]]

        elif line_items[0] == "Operation:":
            if line_items[-1] != "old":
                operation = line_items[-2]
                operand = int(line_items[-1])
                continue
            operation = "^"
            operand = None

        elif line_items[0] == "Test:":
            test_operand = int(line_items[-1])
        elif line_items[1] == "true:":
            true_target_index = int(line_items[-1])

        elif line_items[1] == "false:":
            false_target_index = int(line_items[-1])
        else:
            continue

    # last monkey
    monkeys.append(items)
    operations.append(operation)
    operands.append(operand)
    test.append(test_operand)
    true.append(true_target_index)
    false.append(false_target_index)

    return monkeys, operations, operands, test, true, false


def inspect(item, operation, operand):
    if operation == "*":
        return item * operand

    if operation == "+":
        return item + operand

    return item * item


def play_rounds(rounds: int, part: int, lcd: int | None):

    traffic: list[int] = [0] * len(monkeys)
    for _ in range(rounds):

        for m in range(len(monkeys)):
            len_items = len(monkeys[m])
            traffic[m] += len_items
            for _ in range(len_items):
                item = monkeys[m].pop(0)
                item = inspect(item, operations[m], operands[m])

                if part == 1:
                    item = item // 3
                else:
                    item = item % lcd

                outcome = item % test[m] == 0
                target = true[m] if outcome else false[m]
                monkeys[target].append(item)
    return traffic


# part 1
monkeys, operations, operands, test, true, false = read_input()
sorted_traffic = sorted(play_rounds(20, 1, None))
print(sorted_traffic[-1] * sorted_traffic[-2])  # 98280

# part 2
"""
1) Given list of numbers t_i: 
x is divisible by the product of t_is iff it's divisible by every t_i
x % prod(t_i) == 0 <=> x % t_i == 0 for every t_i in list
So, if x is divisible by prod(t_i), it must be divisible by any given t_i.

2) 
- ((x % p) + a) % p == (x % p + a % p) % p == (x + a) % p, for all a
- ((x % p) * a) % p == (x % p * a % p) % p == (x * a) % p, for all a
- ((x % p) * x) % p == (x % p * x % p) % p == (x * x) % p, for all a

By 1), 2) we can keep track of x % p, where p is product of all test numbers,
and we 'll be able to tell if an item is divisible by any test number
"""
monkeys, operations, operands, test, true, false = read_input()
prod = 1
for t in test:
    prod *= t
sorted_traffic = sorted(play_rounds(10000, 2, prod))
print(sorted_traffic[-1] * sorted_traffic[-2])  # 17673687232
