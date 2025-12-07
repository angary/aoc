from functools import lru_cache


def task1(lines: list[str], start: int) -> int:
    beams = {start}
    splits = 0
    for line in lines:
        hits = {b for b in beams if line[b] == "^"}
        splits += len(hits)
        beams = (beams - hits) | {b + delta for b in hits for delta in (-1, 1)}
    return splits


def task2(lines: list[str], start: int) -> int:
    @lru_cache
    def dfs(b: int, depth: int):
        if depth == len(lines):
            return 1
        if lines[depth][b] == "^":
            return dfs(b - 1, depth + 1) + dfs(b + 1, depth + 1)
        return dfs(b, depth + 1)
    return dfs(start, 0)


def main():
    with open("input.txt") as f:
        lines = [l.strip() for l in f.readlines()]
        start = lines[0].find("S")
        print(task1(lines, start))
        print(task2(lines, start))


if __name__ == "__main__":
    main()
