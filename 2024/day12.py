COORD = tuple[int, int]
DELTAS = [(-1, 0), (0, 1), (1, 0), (0, -1)]
UP, LEFT, DOWN, RIGHT = 0, 1, 2, 3 # cbbs enum

def in_bounds(farm: list[str], i: int, j: int) -> bool:
    return 0 <= i < len(farm) and 0 <= j < len(farm[0])


def get_region(farm: list[str], start: COORD, seen: set[COORD]) -> set[COORD]:
    i1, j1 = start
    res = seen
    for di, dj in DELTAS:
        i2, j2 = i1 + di, j1 + dj
        if in_bounds(farm, i2, j2) and farm[i1][j1] == farm[i2][j2] and (i2, j2) not in seen:
            res.add((i2, j2))
            res = res.union(get_region(farm, (i2, j2), res))
    return res


def get_regions(farm: list[str]) -> list[set[COORD]]:
    visited = set()
    regions = []
    for i, row in enumerate(farm):
        for j in range(len(row)):
            if (i, j) not in visited:
                region = get_region(farm, (i, j), {(i, j)})
                regions.append(region)
                visited = visited.union(region)
    return regions


def get_region_neighbours(farm: list[str], region: set[COORD], i: int, j: int) -> list[COORD]:
    adjacent = [(i + di, j + dj) for di, dj in DELTAS]
    return [a for a in adjacent if in_bounds(farm, *a) and a in region]


def get_edge(
    farm: list[str],
    outline: list[set[tuple[COORD, int]]],
    start: tuple[COORD, int],
    seen: set[tuple[COORD, int]]
) -> set:
    def explore_edge(edge: tuple[COORD, int]) -> None:
        if edge not in seen and edge in outline:
            seen.add(edge)
            get_edge(farm, outline, edge, seen)

    (i, j), direction = start
    if not in_bounds(farm, i, j):
        return set()
    res = seen
    if direction in (UP, DOWN):
        explore_edge(((i + DELTAS[LEFT][0], j + DELTAS[LEFT][1]), direction))
        explore_edge(((i + DELTAS[RIGHT][0], j + DELTAS[RIGHT][1]), direction))

    else:
        explore_edge(((i + DELTAS[UP][0], j + DELTAS[UP][1]), direction))
        explore_edge(((i + DELTAS[DOWN][0], j + DELTAS[DOWN][1]), direction))
    return res


def get_edges(farm: list[str], region: set[COORD]) -> list[set[tuple[COORD, int]]]:
    outline: list[tuple[COORD, int]] = []
    for i, j in region:
        for direction, (di, dj) in enumerate(DELTAS):
            adjacent = (i + di, j + dj)
            if not in_bounds(farm, *adjacent) or not adjacent in region:
                outline.append(((i, j), direction))
    visited = set()
    edges = []
    for start in outline:
        if start not in visited:
            edge = get_edge(farm, outline, start, {start})
            edges.append(edge)
            visited = visited.union(edge)
    return edges

        
def calculate_cost1(farm: list[str], region: set[COORD]):
    area = len(region)
    perimeter = 4 * area - sum(len(get_region_neighbours(farm, region, i, j)) for (i, j) in region)
    return area * perimeter


def calculate_cost2(farm: list[str], region: set[COORD]):
    area = len(region)
    edges = get_edges(farm, region)
    return area * len(edges)


def task1(farm: list[str]) -> int:
    return sum(calculate_cost1(farm, region) for region in get_regions(farm))


def task2(farm: list[str]) -> int:
    return sum(calculate_cost2(farm, region) for region in get_regions(farm))


def main():
    with open("input.txt") as f:
        farm = [l.strip() for l in f.readlines()]
        print(task1(farm))
        print(task2(farm))


if __name__ == "__main__":
    main()
