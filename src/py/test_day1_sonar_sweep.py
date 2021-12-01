from math import inf


def part1(numbers):
    prev = inf
    acc = 0
    for n in numbers:
        if n > prev:
            acc += 1
        prev = n

    return acc


def part2(nums):
    sums = [sum(nums[i : i + 3]) for i in range(len(nums) - 3 + 1)]
    return part1(sums)


def test_day1_part1_sample():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_sample.txt"
    ) as f:
        numbers = [int(s.rstrip("\n")) for s in f.readlines()]
        assert part1(numbers) == 7


def test_day1_part1_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_submission.txt"
    ) as f:
        numbers = [int(s.rstrip("\n")) for s in f.readlines()]
        assert part1(numbers) == 1665


def test_day1_part2_sample():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_sample.txt"
    ) as f:
        numbers = [int(s.rstrip("\n")) for s in f.readlines()]
        assert part2(numbers) == 5


def test_day1_part2_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_submission.txt"
    ) as f:
        numbers = [int(s.rstrip("\n")) for s in f.readlines()]
        assert part2(numbers) == 1702
