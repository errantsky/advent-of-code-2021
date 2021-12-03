def part1(inp):
    counts = find_count(inp)
    gamma_bits = ["1" if counts[i] > len(inp) / 2 else "0" for i in range(len(counts))]
    epsilon_bits = ["0" if bit == "1" else "1" for bit in gamma_bits]
    gamma = int("".join(gamma_bits), 2)
    epsilon = int("".join(epsilon_bits), 2)
    return gamma * epsilon


def find_count(numbers):
    counts = [0] * len(numbers[0])
    for num in numbers:
        for i in range(len(num)):
            if num[i] == "1":
                counts[i] += 1

    return counts


def part2(inp):
    counts = find_count(inp)
    candidates = inp[:]
    for i in range(len(inp[0])):
        if len(candidates) == 1:
            break
        if counts[i] >= len(candidates) / 2:
            candidates = [cd for cd in candidates if cd[i] == "1"]
            counts = find_count(candidates)
        else:
            candidates = [cd for cd in candidates if cd[i] == "0"]
            counts = find_count(candidates)

    o2 = int(candidates.pop(), 2)

    counts = find_count(inp)
    candidates = inp[:]
    for i in range(len(inp[0])):
        if len(candidates) == 1:
            break
        if counts[i] >= len(candidates) / 2:
            candidates = [cd for cd in candidates if cd[i] == "0"]
            counts = find_count(candidates)
        else:
            candidates = [cd for cd in candidates if cd[i] == "1"]
            counts = find_count(candidates)

    co2 = int(candidates.pop(), 2)

    return o2 * co2


def test_day3_part1_sample():
    with open("/Users/sep/CLionProjects/aoc-2021/src/test_files/day3_sample.txt") as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part1(inp) == 198


def test_day3_part1_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day3_submission.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part1(inp) == 2583164


def test_day3_part2_sample():
    with open("/Users/sep/CLionProjects/aoc-2021/src/test_files/day3_sample.txt") as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part2(inp) == 230


def test_day3_part2_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day3_submission.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part2(inp) == 2784375
