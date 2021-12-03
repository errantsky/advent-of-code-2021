def part1(inp):


def part2(inp):


def test_day4_sample():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day4_sample.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part1(inp) == 7


def test_day4_submission():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day4_submission.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part1(inp) == 1665


def test_day4_part2_sample():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day4_sample.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part2(inp) == 5


def test_day4_part2_submission():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day4_submission.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part2(numbers) == 1702
