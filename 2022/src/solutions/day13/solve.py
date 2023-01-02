from functools import cmp_to_key


def parse(input: str) -> list:
    packets = []
    for line in input.splitlines():
        if line != "":
            packets.append(eval(line))
    return packets


def compare(left, right):
    if type(left) == int and type(right) == int:
        if left > right:
            return 1
        elif left < right:
            return -1
        else:
            return 0
    elif type(left) == list and type(right) == list:
        return compare_array(left, right)
    else:
        left = [left] if type(left) == int else left
        right = [right] if type(right) == int else right
        return compare_array(left, right)


def compare_array(left, right):
    for i in range(0, max(len(left), len(right))):
        if i > len(right) - 1:
            return 1
        elif i > len(left) - 1:
            return -1
        res = compare(left[i], right[i])
        if res != 0:
            return res
    return 0


def ordered_pairs(input):
    ordered = 0
    p = 0
    for i in range(0, len(input), 2):
        p += 1
        if compare_array(input[i], input[i + 1]) == -1:
            ordered += p
    return ordered


def decoder_key(input):
    key = 1
    input.append([[2]])
    input.append([[6]])
    ordered_packets = sorted(
        input, key=cmp_to_key(lambda left, right: compare_array(left, right))
    )
    for i, packet in enumerate(ordered_packets):
        if packet == [[2]] or packet == [[6]]:
            key *= i + 1
    return key


def solve(input):
    input = parse(input)
    return ordered_pairs(input), decoder_key(input)
