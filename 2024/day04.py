XMAS = "XMAS"
DXS = [1, 1, 0, -1, -1, -1, 0, 1]
DYS = [0, -1, -1, -1, 0, 1, 1, 1]

def in_bounds(i: int, j: int, grid: list[str]):
    return 0 <= i < len(grid) and 0 <= j < len(grid[0])


def matches1(grid: list[str], i1: int, j1: int, dx: int, dy: int) -> bool:
    for k in range(len(XMAS)):
        i2, j2 = i1 + dx * k, j1 + dy * k
        if not in_bounds(i2, j2, grid) or grid[i2][j2] != XMAS[k]:
            return False
    return True


def task1(grid: list[str]) -> int:
    return sum(matches1(grid, i, j, dx, dy) 
        for i in range(len(grid))
        for j in range(len(grid[0]))
        for dx, dy in zip(DXS, DYS))


def is_mas(s: str) -> bool:
    return s == XMAS[1:] or s[::-1] == XMAS[1:]


def matches2(grid: list[str], i: int, j: int) -> bool:
    l = grid[i-1][j-1] + grid[i][j] + grid[i+1][j+1]
    r = grid[i-1][j+1] + grid[i][j] + grid[i+1][j-1]
    return is_mas(l) and is_mas(r)


def task2(grid: list[str]) -> int:
    return sum(matches2(grid, i, j) for i in range(1, len(grid)-1) for j in range(1, len(grid[0])-1))
    

def main():
    with open("input.txt") as f:
        grid = [line.strip() for line in f.readlines()]
        print(task1(grid))
        print(task2(grid))


if __name__ == "__main__":
    main()
