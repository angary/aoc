from bisect import bisect_left, bisect_right
from collections import defaultdict
from enum import Enum


DELTAS: list[tuple[int, int]] = [(-1, 0), (0, 1), (1, 0), (0, -1)]
EMPTY: str = "."
OBSTACLE: str = "#"


class Direction(Enum):
    UP = 0
    RIGHT = 1
    DOWN = 2
    LEFT = 3


def find_start(grid: list[str]) -> tuple[int, int]:
    for i, row in enumerate(grid):
        if (j := row.find("^")) != -1:
            return (i, j)


def next_direction(direction: int) -> int:
    return (direction + 1) % 4


def in_bounds(grid: list[str], pos: tuple[int, int]) -> bool:
    return 0 <= pos[0] < len(grid) and 0 <= pos[1] < len(grid[0])


def is_obstacle(grid: list[str], pos: tuple[int, int]) -> bool:
    return in_bounds(grid, pos) and grid[pos[0]][pos[1]] == OBSTACLE


def has_cycle(grid: list[str], pos: tuple[int, int]) -> bool:
    direction = 0
    seen = set()
    while in_bounds(grid, pos):
        delta = DELTAS[direction]
        next_pos = (pos[0] + delta[0], pos[1] + delta[1])
        if is_obstacle(grid, next_pos):
            if ((pos, direction) in seen):
                return True
            seen.add((pos, direction))
            direction = next_direction(direction)
        else:
            pos = next_pos
    return False


def get_visited(grid: list[str], pos: tuple[int, int]) -> set[tuple[int, int]]:
    direction = 0
    visited = set()
    while in_bounds(grid, pos):
        visited.add(pos)
        delta = DELTAS[direction]
        next_pos = (pos[0] + delta[0], pos[1] + delta[1])
        if is_obstacle(grid, next_pos):
            direction = next_direction(direction)
        else:
            pos = next_pos
    return visited



def task1(grid: list[str], start: tuple[int, int]) -> int:
    return len(get_visited(grid, start))


def task2(grid: list[str], start: tuple[int, int]) -> int:
    count = 0
    for (i, j) in get_visited(grid, start):
        if grid[i][j] == ".":
            string = grid[i]
            grid[i] = string[:j] + "#" + string[j+1:]
            count += has_cycle(grid, start)
            grid[i] = string
    return count


def get_next_position_optimised(
    pos: tuple[int, int],
    direction: int,
    obstacle_ys: list[int],
    obstacle_xs: list[int]
) -> tuple[int, int] | None:
    y, x = pos
    match direction:
        case Direction.UP.value:  # (max row below current, same column)
            idx = bisect_left(obstacle_ys, y)
            if idx > 0:
                return (obstacle_ys[idx - 1] + 1, x)
        case Direction.RIGHT.value:  # (same row, min col above current)
            idx = bisect_right(obstacle_xs, x)
            if idx < len(obstacle_xs):
                return (y, obstacle_xs[idx] - 1)
        case Direction.DOWN.value:  # (min row above current, same column)
            idx = bisect_right(obstacle_ys, y)
            if idx < len(obstacle_ys):
                return (obstacle_ys[idx] - 1, x)
        case Direction.LEFT.value:  # (same row, max col below current)
            idx = bisect_left(obstacle_xs, x)
            if idx > 0:
                return (y, obstacle_xs[idx - 1] + 1)
    return None


def has_cycle_optimised(
    grid: list[str],
    pos: tuple[int, int],
    new_obstacle: tuple[int, int],
    row_indexes_of_column: dict[int, list[int]],
    col_indexes_of_row: dict[int, list[int]]
) -> bool:
    # Add new obstacle into our obstacle maps
    i, j = new_obstacle
    row_indexes_of_column = row_indexes_of_column.copy()
    col_indexes_of_row = col_indexes_of_row.copy()
    row_indexes_of_column[j] = sorted(row_indexes_of_column.get(j) + [i])
    col_indexes_of_row[i] = sorted(col_indexes_of_row.get(i) + [j])

    direction = 0
    seen = set()
    while in_bounds(grid, pos):
        i, j = pos
        row_indexes = row_indexes_of_column.get(j)
        col_indexes = col_indexes_of_row.get(i)
        if not (pos := get_next_position_optimised(pos, direction, row_indexes, col_indexes)):
            return False
        if ((pos, direction) in seen):
            return True
        seen.add((pos, direction))
        direction = next_direction(direction)
    return False


def task2_optimised(grid: list[str], start: tuple[int, int]) -> int:
    # Generation of map of column index -> list of row indexes where there is an obstacle and vise versa
    obstacle_cols = defaultdict(list)
    obstacle_rows = defaultdict(list)
    for i, row in enumerate(grid):
        for j, cell in enumerate(row):
            if cell == OBSTACLE:
                obstacle_cols[j].append(i)
                obstacle_rows[i].append(j)
    obstacle_cols = {k: sorted(v) for k, v in obstacle_cols.items()}
    obstacle_rows = {k: sorted(v) for k, v in obstacle_rows.items()}
    count = 0
    for (i, j) in get_visited(grid, start):
        if grid[i][j] == EMPTY:
            count += has_cycle_optimised(grid, start, (i, j), (obstacle_cols), (obstacle_rows))
    return count
    

def main():
    with open("input.txt") as f:
        grid = [l.strip() for l in f.readlines()]
        start = find_start(grid)
        print(task1(grid, start))
        print(task2_optimised(grid, start))


if __name__ == "__main__":
    main()
