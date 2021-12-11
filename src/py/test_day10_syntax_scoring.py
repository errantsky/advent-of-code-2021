import pathlib
from collections import deque

SAMPLE_PATH = "../test_files/day10_sample.txt"
SUBMISSION_PATH = "../test_files/day10_submission.txt"

openings = {"<", "{", "[", "("}
closings = {">", "}", "]", ")"}
pairs = {"<": ">", "{": "}", "[": "]", "(": ")"}
corruption_scores_dict = {")": 3, "]": 57, "}": 1197, ">": 25137}
completion_scores_dict = {
    "(": 1,
    "[": 2,
    "{": 3,
    "<": 4,
}


def find_corruption_score(line):
    stack = deque()
    for char in line:
        if char in openings:
            stack.append(char)
        elif char in closings:
            top = stack.pop()
            if char != pairs[top]:
                return corruption_scores_dict[char]
            else:
                continue
        else:
            raise Exception(f"Oops!, wrong character {char}")
    return 0


def part1(lines):
    score = 0
    for line in lines:
        score += find_corruption_score(line)

    return score


def find_completion_score(line):
    stack = deque()
    for char in line:
        if char in openings:
            stack.append(char)
        if char in closings:
            stack.pop()

    completion_score = 0
    while stack:
        leftover = stack.pop()
        completion_score = completion_score * 5 + completion_scores_dict[leftover]

    return completion_score


def part2(lines):
    incomplete_lines = [line for line in lines if find_corruption_score(line) == 0]
    scores = [find_completion_score(line) for line in incomplete_lines]
    scores.sort()

    return scores[len(scores) // 2]


def test_day10_part1_sample():
    file = pathlib.Path(SAMPLE_PATH)
    lines = file.read_text().splitlines()
    assert part1(lines) == 26397


def test_day10_part1_submission():
    file = pathlib.Path(SUBMISSION_PATH)
    lines = file.read_text().splitlines()
    assert part1(lines) == 319329


def test_day10_part2_sample():
    file = pathlib.Path(SAMPLE_PATH)
    lines = file.read_text().splitlines()
    assert part2(lines) == 288957


def test_day10_part2_submission():
    file = pathlib.Path(SUBMISSION_PATH)
    lines = file.read_text().splitlines()
    assert part2(lines) == 3515583998
