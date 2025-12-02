def sum_invalid(is_invalid_func: callable[[str], bool]):
    def wrapper(ranges: list[list[int]]) -> int:
        count = 0
        for r in ranges:
            count += sum(i for i in range(r[0], r[1]) if is_invalid_func(str(i)))
        return count
    return wrapper


@sum_invalid
def task1(s: str) -> bool:
    n = len(s)
    return s[:n//2] == s[n//2:]


@sum_invalid
def task2(s: str) -> bool:
    n = len(s)
    return any(
        len(set(s[i:i+chunk_size] for i in range(0, n, chunk_size))) == 1
        for chunk_size in range(1, n // 2 + 1)
    )


def main():
    with open("input.txt") as f:
        ids = f.readline().strip().split(",")
        ranges = [[int(i) for i in r.split("-")] for r in ids]
        print(task1(ranges))
        print(task2(ranges))


if __name__ == "__main__":
    main()
