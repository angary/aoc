from functools import lru_cache


def joltage(line: str) -> int:
    peak = 0
    for i, c in enumerate(line[:-1]):
        peak = max(peak, int(c) * 10 + int(max(line[i+1:])))
    return peak


@lru_cache
def joltage2(line: str, index: int) -> int:
    if len(line) == 1: # We've reached the last character
        return int(line[0])
    if index == 0: # We can only squeeze in 1 more digit
        return int(max(line))
    keep = (int(line[0]) * (10 ** index)) + joltage2(line[1:], index - 1)
    if index == len(line) - 1: # We cannot ignore the current char
        return keep
    ignore = joltage2(line[1:], index)
    return max(keep, ignore)


def task1(lines: list[str]) -> int:
    return sum(joltage(l) for l in lines)


def task2(lines: list[str]) -> int:
    return sum(joltage2(l, 11) for l in lines)


def main():
    with open("input.txt") as f:
        lines  = [l.strip() for l in f.readlines()]
        print(task1(lines))
        print(task2(lines))


if __name__ == "__main__":
    main()