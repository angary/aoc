DIRECTIONS = "^>v<"
DELTAS = [(-1, 0), (0, 1), (1, 0), (0, -1)]
BOX_1 = "O"
BOX_2 = "[]"
OBSTACLE = "#"
WORKER = "@"
EMPTY = "."
P2_MAPPING = {
    EMPTY: [EMPTY, EMPTY],
    OBSTACLE: [OBSTACLE, OBSTACLE],
    BOX_1: [BOX_2[0], BOX_2[1]],
    WORKER: [WORKER, EMPTY],
}


def parse_directions(lines: list[str]) -> list[tuple[int, int]]:
    return [DIRECTIONS.index(c) for l in lines if any(m in l for m in DIRECTIONS) for c in l]
    
    
def parse_warehouse1(lines: list[str]) -> list[list[str]]:
    return [[c for c in l] for l in lines if OBSTACLE in l]


def parse_warehouse2(lines: list[str]) -> list[list[str]]:
    return [[char for c in l for char in P2_MAPPING[c]] for l in lines if OBSTACLE in l]


def get_worker(warehouse: list[list[str]]) -> tuple[int, int]:
    return next((i, row.index(WORKER)) for i, row in enumerate(warehouse) if WORKER in row)


# Returns a tuple of [left_box_pos, right_box_pos]
def get_box_pos(char: str, i: int, j: int) -> tuple[tuple[int, int], tuple[int, int]]:
    return ((i, j), (i, j+1)) if char == BOX_2[0] else ((i, j-1), (i, j))


def vertical(delta: tuple[int, int]) -> bool:
    return delta in ((1, 0), (-1, 0))


def blocked1(warehouse: list[list[str]], pos: tuple[int, int], delta: tuple[int, int]) -> bool:
    i, j = pos[0] + delta[0], pos[1] + delta[1]
    while warehouse[i][j] == BOX_1:
        i, j = i + delta[0], j + delta[1]
    return warehouse[i][j] == OBSTACLE


def push1(warehouse: list[list[str]], pos: tuple[int, int], delta: tuple[int, int]):
    i, j = pos
    di, dj = delta
    ni, nj = i + di, j + dj
    # Move the worker
    warehouse[i][j] = EMPTY
    warehouse[ni][nj] = WORKER
    # Move the box
    ni, nj = ni + di, nj + dj
    while warehouse[ni][nj] == BOX_1:
        ni, nj = ni + di, nj + dj
    warehouse[ni][nj] = BOX_1


def blocked2(warehouse: list[list[str]], pos: tuple[int, int], delta: tuple[int, int]) -> bool:
    i1, j1 = pos[0] + delta[0], pos[1] + delta[1]
    if warehouse[i1][j1] == OBSTACLE:
        return True
    if warehouse[i1][j1] == EMPTY:
        return False
    if vertical(delta):
        (i1, j1), (i2, j2) = get_box_pos(warehouse[i1][j1], i1, j1)
        return blocked2(warehouse, (i1, j1), delta) or blocked2(warehouse, (i2, j2), delta)
    return blocked2(warehouse, (i1, j1), delta)


def push2(warehouse: list[list[str]], pos: tuple[int, int], delta: tuple[int, int]):
    i, j = pos
    di, dj = delta
    ni, nj = i + di, j + dj
    # Move the boxes
    push_box(warehouse, (ni, nj), delta)
    # Move the worker
    warehouse[i][j], warehouse[ni][nj] = EMPTY, WORKER


def push_box(warehouse: list[list[str]], pos: tuple[int, int], delta: tuple[int, int]):
    i, j = pos
    di, dj = delta
    box_positions = get_box_pos(warehouse[i][j], i, j)
    (i1, j1), (i2, j2) = box_positions
    if vertical(delta):
        if any(blocked2(warehouse, pos, delta) for pos in box_positions):
            return
        # If any of the next positions contain a box then push that first
        ni1, nj1 = i1 + di, j1 + dj
        ni2, nj2 = i2 + di, j2 + dj
        if warehouse[ni1][nj1] in BOX_2:
            push_box(warehouse, (ni1, nj1), delta)
        if warehouse[ni2][nj2] in BOX_2:
            push_box(warehouse, (ni2, nj2), delta)
        # Push the current box
        warehouse[ni1][nj1] = warehouse[i1][j1]
        warehouse[ni2][nj2] = warehouse[i2][j2]
        warehouse[i1][j1] = EMPTY
        warehouse[i2][j2] = EMPTY
        return
    if blocked2(warehouse, (i, j), delta):
        return
    if delta == (0, -1): # Left
        if warehouse[i][j1-1] == BOX_2[1]:
            push_box(warehouse, (i, j1-1), delta)
        warehouse[i][j1 - 1], warehouse[i][j2 - 1] = BOX_2[0], BOX_2[1]
        warehouse[i][j2] = EMPTY
    elif delta == (0, 1): # Right
        if warehouse[i][j2+1] == BOX_2[0]:
            push_box(warehouse, (i, j2+1), delta)
        warehouse[i][j2 + 1], warehouse[i][j1 + 1] = BOX_2[1], BOX_2[0]
        warehouse[i][j1] = EMPTY


def move(
    warehouse: list[list[str]],
    delta: tuple[int, int],
    blocked_fn: callable,
    push_fn: callable
):
    i1, j1 = get_worker(warehouse)
    i2, j2 = i1 + delta[0], j1 + delta[1]
    if warehouse[i2][j2] == OBSTACLE:
        return
    if warehouse[i2][j2] == EMPTY:
        warehouse[i1][j1], warehouse[i2][j2] = EMPTY, WORKER
        return
    if not blocked_fn(warehouse, (i1, j1), delta):
        push_fn(warehouse, (i1, j1), delta)


def calculate_gps(warehouse: list[list[str]], obstacle: str) -> int:
    gps_sum = 0
    for i, row in enumerate(warehouse):
        for j, val in enumerate(row):
            if val == obstacle:
                gps_sum += 100 * i + j
    return gps_sum


def task1(warehouse: list[list[str]], directions: list[int]) -> int:
    for direction in directions:
        move(warehouse, DELTAS[direction], blocked1, push1)
    return calculate_gps(warehouse, BOX_1)


def task2(warehouse: list[list[str]], directions: list[int]) -> int:
    for direction in directions:
        move(warehouse, DELTAS[direction], blocked2, push2)
    return calculate_gps(warehouse, BOX_2[0])


def main():
    with open("input.txt") as f:
        lines = [l.strip() for l in f.readlines()]
        directions = parse_directions(lines)
        warehouse1 = parse_warehouse1(lines)
        print(task1(warehouse1, directions))

        warehouse2 = parse_warehouse2(lines)
        print(task2(warehouse2, directions))


if __name__ == "__main__":
    main()
