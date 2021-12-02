def part1():



def test_day1_part1_sample():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_sample.txt"
    ) as f:
        cmd_pairs = [s.rstrip("\n").split(' ') for s in f.readlines()]
        assert part1(numbers) == 150


def test_day1_part1_submission():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day1_part1_submission.txt"
    ) as f:
        assert part1(numbers) == 1746616


def test_day1_part2_sample():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day2_sample.txt"
    ) as f:
        assert part2(numbers) == 900


def test_day1_part2_submission():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day2_submission.txt"
    ) as f:
        assert part2(numbers) == 1741971043
