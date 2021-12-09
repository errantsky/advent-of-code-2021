def part1(inp):
    grid = [[0] * 1100 for _ in range(1100)]
    for line in inp:
        x1, y1, x2, y2 = [int(x) for x in line.replace(' -> ', ',').split(',')]
        if x1 == x2:
            low = min(y1, y2)
            high = max(y1, y2)
            for y in range(low, high + 1):
                grid[x1][y] += 1

        elif y1 == y2:
            low = min(x1, x2)
            high = max(x1, x2)
            for x in range(low, high + 1):
                grid[x][y1] += 1
        else:
            if x1 < x2 and y1 < y2:
                while x1 != x2 + 1 and y1 != y2 + 1:
                    grid[x1][y1] += 1
                    x1 += 1
                    y1 += 1
            elif x1 > x2 and y1 > y2:
                while x1 + 1 != x2 and y1 + 1 != y2:
                    grid[x2][y2] += 1
                    x2 += 1
                    y2 += 1
            elif x1 < x2 and y1 > y2:
                while x1 != x2 + 1 and y1 != y2 - 1:
                    grid[x1][y1] += 1
                    x1 += 1
                    y1 -= 1
            elif x1 > x2 and y1 < y2:
                while x1 != x2 - 1 and y1 != y2 + 1:
                    grid[x1][y1] += 1
                    x1 -= 1
                    y1 += 1

    acc = 0
    for row in grid:
        for cell in row:
            if cell >= 2:
                acc += 1

    return acc


def test_day9_sample():
    with open("/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_sample.txt") as f:
        inp = [s.rstrip("\n").lstrip() for s in f.readlines()]
        assert part1(inp) == 5


def test_day9_submission():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_submission.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part1(inp) == 16674



def test_day9_part2_sample():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_sample.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part1(inp) == 12


def test_day9_part2_submission():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_submission.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part1(inp) == 7075
