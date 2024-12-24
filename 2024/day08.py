from collections import defaultdict
from itertools import count


EMPTY = "."


def get_satellites(city: list[str]) -> list[list[tuple[int, int]]]:
    satellites = defaultdict(list)
    for y, row in enumerate(city):
        for x, char in enumerate(row):
            if char != EMPTY:
                satellites[char].append((y, x))
    return satellites.values()


def in_bounds(city: list[str], y: int, x: int) -> bool:
    return 0 <= y < len(city) and 0 <= x < len(city[0])


def generate_antinodes(y1: int, x1: int, y2: int, x2: int, scale=1) -> tuple[tuple[int, int], tuple[int, int]]:
    dy = scale * (y2 - y1)
    dx = scale * (x2 - x1)
    return (y1 - dy, x1 - dx), (y2 + dy, x2 + dx)


def add_antinode(antinodes: set[tuple[int, int]], city: list[str], an: tuple[int, int]) -> bool:
    if in_bounds(city, *an):
        antinodes.add(an)
        return True
    return False


def task1(city: list[str], satellites: list[list[tuple[int, int]]]) -> int:
    antinodes = set()
    for coords in satellites:
        for i, (y1, x1) in enumerate(coords):
            for y2, x2 in coords[i + 1:]:
                for an in generate_antinodes(y1, x1, y2, x2):
                    add_antinode(antinodes, city, an)
    return len(antinodes)
    

def task2(city: list[str], satellites: list[list[tuple[int, int]]]) -> int:
    antinodes = set()
    for coords in satellites:
        for i, (y1, x1) in enumerate(coords):
            for (y2, x2) in coords[i+1:]:
                for scale in count():
                    an1, an2 = generate_antinodes(y1, x1, y2, x2, scale)
                    if not (add_antinode(antinodes, city, an1) | add_antinode(antinodes, city, an2)):
                        break
    return len(antinodes)


def main():
    with open("input.txt") as f:
        city = [l.strip() for l in f.readlines()]
        satellites = get_satellites(city)
        print(task1(city, satellites))
        print(task2(city, satellites))
    

if __name__ == "__main__":
    main()
