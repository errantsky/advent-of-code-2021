import pathlib
import re

SAMPLE_PATH = "../test_files/day17_sample.txt"
SUBMISSION_PATH = "../test_files/day17_submission.txt"


def part1(lines):
    pattern = re.compile(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")
    x_start, x_stop, y_start, y_stop = re.match(pattern, lines[0]).groups()


# def part2(lines):


def test_day17_part1_sample():
    file = pathlib.Path(SAMPLE_PATH)
    lines = file.read_text().splitlines()
    assert part1(lines) == 45


def test_day17_part1_submission():
    file = pathlib.Path(SUBMISSION_PATH)
    lines = file.read_text().splitlines()
    assert part1(lines) == 319329


# def test_day17_part2_sample():
#     file = pathlib.Path(SAMPLE_PATH)
#     lines = file.read_text().splitlines()
#     assert part2(lines) == 288957
#
#
# def test_day17_part2_submission():
#     file = pathlib.Path(SUBMISSION_PATH)
#     lines = file.read_text().splitlines()
#     assert part2(lines) == 3515583998
