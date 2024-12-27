from functools import cache

GRID = list[list[int]]
COORD = tuple[int, int]
DELTAS = [(-1, 0), (0, 1), (1, 0), (0, -1)]

def find_starts(grid: GRID) -> list[COORD]:
    res = []
    for i, row in enumerate(grid):
        for j, value in enumerate(row):
            if value == 0:
                res.append((i, j))
    return res


def in_bounds(grid: GRID, i: int, j: int) -> bool:
    return 0 <= i < len(grid) and 0 <= j < len(grid[0])


def find_score1(grid: GRID, start: COORD) -> set[COORD]:
    i1, j1 = start
    if grid[i1][j1] == 9:
        return {(i1, j1)}
    nines_found = set()
    for di, dj in DELTAS:
        i2, j2 = i1 + di, j1 + dj
        if in_bounds(grid, i2, j2) and grid[i2][j2] == grid[i1][j1] + 1:
            nines_found = nines_found.union(find_score1(grid, (i2, j2)))
    return nines_found
        
def find_score2(grid: GRID, start: COORD) -> int:
    i1, j1 = start
    if grid[i1][j1] == 9:
        return 1
    score = 0
    for di, dj in DELTAS:
        i2, j2 = i1 + di, j1 + dj
        if in_bounds(grid, i2, j2) and grid[i2][j2] == grid[i1][j1] + 1:
            score += find_score2(grid, (i2, j2))
    return score

def task1(grid: GRID) -> int:
    return sum(len(find_score1(grid, start)) for start in find_starts(grid))

def task2(grid: GRID) -> int:
    return sum(find_score2(grid, start) for start in find_starts(grid))

def main():
    with open("input.txt") as f:
        grid = [[int(c) for c in l.strip()] for l in f.readlines()]
        print(task1(grid))
        print(task2(grid))


if __name__ == "__main__":
    main()
