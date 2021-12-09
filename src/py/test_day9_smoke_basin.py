from functools import reduce
from collections import deque


def find_valid_neighbors(grid, i, j):
    n_rows = len(grid)
    n_cols = len(grid[0])
    deltas = [(0, 1), (1, 0), (-1, 0), (0, -1)]
    neighbors = []
    for di, dj in deltas:
        if 0 <= i + di < n_rows and 0 <= j + dj < n_cols:
            neighbors.append((i + di, j + dj))

    return neighbors


def find_low_points(grid):
    n_rows = len(grid)
    n_cols = len(grid[0])
    low_points = []
    for i in range(n_rows):
        for j in range(n_cols):
            neighbors = find_valid_neighbors(grid, i, j)
            if all([grid[i][j] < grid[ni][nj] for ni, nj in neighbors]):
                low_points.append((i, j))

    return low_points


def part1(grid):
    low_points = find_low_points(grid)
    return sum([1 + grid[i][j] for i, j in low_points])


def part2(grid, low_points):
    basin_lists = []
    for li, lj in low_points:
        # With a normal DFS, some nodes will be visited multiple times, as grids have circular connections
        basin = {(li, lj)}
        visited = set()
        to_visit = deque([(li, lj)])
        while to_visit:
            i, j = to_visit.pop()
            visited.add((i, j))
            for ni, nj in find_valid_neighbors(grid, i, j):
                if (
                    (ni, nj) not in visited
                    and grid[ni][nj] != 9
                    and grid[ni][nj] > grid[i][j]
                ):
                    to_visit.append((ni, nj))
                    basin.add((ni, nj))

        basin_lists.append(basin)

    basin_sizes = sorted([len(bl) for bl in basin_lists])[-3:]
    return reduce(lambda left, right: left * right, basin_sizes)


def test_day9_sample():
    with open("/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_sample.txt") as f:
        inp = [[int(x) for x in s.rstrip("\n").lstrip()] for s in f.readlines()]
        assert part1(inp) == 15


def test_day9_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_submission.txt"
    ) as f:
        inp = [[int(x) for x in s.rstrip("\n").lstrip()] for s in f.readlines()]
        assert part1(inp) == 475


def test_day9_part2_sample():
    with open("/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_sample.txt") as f:
        inp = [[int(x) for x in s.rstrip("\n").lstrip()] for s in f.readlines()]
        low_points = find_low_points(inp)
        assert part2(inp, low_points) == 1134


def test_day9_part2_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day9_submission.txt"
    ) as f:
        inp = [[int(x) for x in s.rstrip("\n").lstrip()] for s in f.readlines()]
        low_points = find_low_points(inp)
        assert part2(inp, low_points) == 1092012
