import pathlib
from collections import defaultdict
import math
import heapq

SAMPLE_PATH = "../test_files/day15_sample.txt"
SUBMISSION_PATH = "../test_files/day15_submission.txt"


def valid_neighbors(x, y, n_rows, n_cols):
    neighbors = []
    deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    for dx, dy in deltas:
        if 0 <= x + dx < n_rows and 0 <= y + dy < n_cols:
            neighbors.append((x + dx, y + dy))

    return neighbors


def part1(lines):
    adj_mat = [[int(x) for x in l] for l in lines]
    dist = defaultdict(lambda: math.inf)
    prev = dict()
    visited = set()
    dist[(0, 0)] = 0
    heap = [(0, (0, 0))]

    while heap:
        d, (x, y) = heapq.heappop(heap)
        for nx, ny in valid_neighbors(x, y, len(adj_mat), len(adj_mat[0])):
            if (nx, ny) in visited:
                continue

            if dist[(nx, ny)] > dist[(x, y)] + adj_mat[nx][ny]:
                dist[(nx, ny)] = dist[(x, y)] + adj_mat[nx][ny]
                prev[(nx, ny)] = (x, y)
                visited.add((nx, ny))
                heapq.heappush(heap, (dist[(nx, ny)], (nx, ny)))

    return dist[(len(adj_mat) - 1, len(adj_mat[0]) - 1)]


def part2(lines):
    adj_mat = [[int(x) for x in l] for l in lines]
    n_rows = len(adj_mat)
    n_cols = len(adj_mat[0])
    graph = dict()
    for i in range(n_rows):
        for j in range(n_cols):
            graph[(i, j)] = adj_mat[i][j]

    del adj_mat

    for i in range(n_rows * 5):
        for j in range(n_cols * 5):
            if (i, j) in graph:
                continue

            val = graph[(i % n_rows, j % n_cols)]
            new_val = val + i // n_rows + j // n_cols
            graph[(i, j)] = new_val % 10 + new_val // 10

    dist = defaultdict(lambda: math.inf)
    prev = dict()
    visited = set()
    dist[(0, 0)] = 0
    heap = [(0, (0, 0))]

    while heap:
        d, (x, y) = heapq.heappop(heap)
        for nx, ny in valid_neighbors(x, y, n_rows * 5, n_cols * 5):
            if (nx, ny) in visited:
                continue

            if dist[(nx, ny)] > dist[(x, y)] + graph[(nx, ny)]:
                dist[(nx, ny)] = dist[(x, y)] + graph[(nx, ny)]
                prev[(nx, ny)] = (x, y)
                visited.add((nx, ny))
                heapq.heappush(heap, (dist[(nx, ny)], (nx, ny)))

    return dist[(n_rows * 5 - 1, n_cols * 5 - 1)]


def test_day15_part1_sample():
    file = pathlib.Path(SAMPLE_PATH)
    lines = file.read_text().splitlines()
    assert part1(lines) == 40


def test_day15_part1_submission():
    file = pathlib.Path(SUBMISSION_PATH)
    lines = file.read_text().splitlines()
    assert part1(lines) == 619


def test_day15_part2_sample():
    file = pathlib.Path(SAMPLE_PATH)
    lines = file.read_text().splitlines()
    assert part2(lines) == 315


def test_day15_part2_submission():
    file = pathlib.Path(SUBMISSION_PATH)
    lines = file.read_text().splitlines()
    assert part2(lines) == 2922
