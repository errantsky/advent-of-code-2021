def part1(inp):
    drawn_numbers = [int(x) for x in inp.pop(0).split(",")]
    boards = []
    for i in range(len(inp) // 6):
        inp.pop(0)
        board = [[int(x) for x in inp.pop(0).split()] for j in range(5)]
        boards.append(board)

    for num in drawn_numbers:
        for board in boards:
            mark_board(board, num)
            if check_for_win(board):
                return num * sum(
                    [sum(x for x in board[i][:] if x != -1) for i in range(len(board))]
                )


def part2(inp):
    drawn_numbers = [int(x) for x in inp.pop(0).split(",")]
    boards = []
    for i in range(len(inp) // 6):
        inp.pop(0)
        board = [[int(x) for x in inp.pop(0).split()] for j in range(5)]
        boards.append(board)

    boards_won = set()
    for num in drawn_numbers:
        for i, board in enumerate(boards):
            if i in boards_won:
                continue
            mark_board(board, num)
            if check_for_win(board):
                boards_won.add(i)
            if len(boards_won) == len(boards):
                return num * sum(
                        [
                            sum(x for x in board[i][:] if x != -1)
                            for i in range(len(board))
                        ]
                    )


def mark_board(board, num):
    for i in range(len(board)):
        for j in range(len(board[0])):
            if board[i][j] == num:
                board[i][j] = -1


def check_for_win(board):
    for row in board:
        if all(x == -1 for x in row):
            return True

    for col in range(len(board[0])):
        if all(board[i][col] == -1 for i in range(len(board))):
            return True

    return False


# def part2(inp):


def test_check_for_win():
    board = [
        [1, 2, 3, 4],
        [1, 2, 3, 4],
        [-1, -1, -1, -1],
        [1, 2, 3, 4],
    ]
    assert check_for_win(board) is True
    board[2] = [1, 1, 1, 1]
    assert check_for_win(board) is False
    board[0][1] = -1
    board[1][1] = -1
    board[2][1] = -1
    board[3][1] = -1
    assert check_for_win(board) is True


def test_day4_sample():
    with open("/Users/sep/CLionProjects/aoc-2021/src/test_files/day4_sample.txt") as f:
        inp = [s.rstrip("\n").lstrip() for s in f.readlines()]
        assert part1(inp) == 4512


def test_day4_submission():
    with open(
        "/Users/sep/CLionProjects/aoc-2021/src/test_files/day4_submission.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part1(inp) == 16674



def test_day4_part2_sample():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day4_sample.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part2(inp) == 1924


def test_day4_part2_submission():
    with open(
            "/Users/sep/CLionProjects/aoc-2021/src/test_files/day4_submission.txt"
    ) as f:
        inp = [s.rstrip("\n") for s in f.readlines()]
        assert part2(inp) == 7075
