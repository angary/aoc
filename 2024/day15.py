
DIRECTIONS = "^>V<"
DELTAS = [(-1, 0), (0, 1), (1, 0), (0, -1)]
BOX = "O"
OBSTACLE = "#"


def task1(warehouse: list[list[str]], directions: list[int]) -> int:
    for w in warehouse:
        print(w)
    print(directions)


def main():
    with open("input.txt") as f:
        warehouse: list[list[str]] = []
        directions: list[int] = []
        for l in f.readlines():
            if OBSTACLE in l:
                warehouse.append([c for c in l])
            elif any(m in l for m in DIRECTIONS):
                directions.extend(DIRECTIONS.index(c) for c in l.strip())
        task1(warehouse, directions)
