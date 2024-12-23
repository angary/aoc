def is_valid(value: int, remaining: list[int], f: callable) -> bool:
    return f(value, remaining[0], remaining[1:])


def solve1(value: int, curr: int, remaining: list[int]) -> bool:
    if not remaining:
        return curr == value
    return solve1(value, curr * remaining[0], remaining[1:]) or solve1(value, curr + remaining[0], remaining[1:])


def solve2(value: int, curr: int, remaining: list[int]) -> bool:
    if not remaining:
        return curr == value
    return (solve2(value, curr * remaining[0], remaining[1:])
        or solve2(value, curr + remaining[0], remaining[1:])
        or solve2(value, int(str(curr) + str(remaining[0])), remaining[1:]))


def task1(values: list[int], nss: list[int]) -> int:
    return sum(v for v, ns in zip(values, nss) if is_valid(v, ns, solve1))


def task2(values: list[int], nss: list[int]) -> int:
    return sum(v for v, ns in zip(values, nss) if is_valid(v, ns, solve2))


def main():
    with open("input.txt") as f:
        equations = [l.strip() for l in f.readlines()]
        values = [int(x.split(":")[0]) for x in equations]
        nums = [[int(x) for x in xs.split(":")[1].split()] for xs in equations]
        print(task1(values, nums))
        print(task2(values, nums))
        
        
if __name__ == "__main__":
    main()
