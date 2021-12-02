def part1(pairs):
    depth = 0
    h_pos = 0
    for cmd, val in pairs:
        val = int(val)
        if cmd == "forward":
            h_pos += val
        elif cmd == "up":
            depth -= val
        elif cmd == "down":
            depth += val
        else:
            raise Exception

    return h_pos * depth


def part2(pairs):
    h_pos = 0
    depth = 0
    aim = 0
    for cmd, val in pairs:
        val = int(val)
        if cmd == "forward":
            depth += aim * val
            h_pos += val
        elif cmd == "up":
            aim -= val
        elif cmd == "down":
            aim += val
        else:
            raise Exception

    return h_pos * depth


def test_day2_sample():
    with open("/Users/sep/CLionProjects/aoc-2021/src/test_files/day2_sample.txt") as f:
        cmd_pairs = [s.rstrip("\n").split(" ") for s in f.readlines()]
        assert part1(cmd_pairs) == 150


def test_day2_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day2_submission.txt"
    ) as f:
        cmd_pairs = [s.rstrip("\n").split(" ") for s in f.readlines()]
        assert part1(cmd_pairs) == 1746616


def test_day1_part2_sample():
    with open("/Users/sep/CLionProjects/aoc-2021/src/test_files/day2_sample.txt") as f:
        cmd_pairs = [s.rstrip("\n").split(" ") for s in f.readlines()]
        assert part2(cmd_pairs) == 900


def test_day1_part2_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day2_submission.txt"
    ) as f:
        cmd_pairs = [s.rstrip("\n").split(" ") for s in f.readlines()]
        assert part2(cmd_pairs) == 1741971043
