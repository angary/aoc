def is_fresh(i: int, ranges: list[list[int]]):
    for [l, r] in ranges:
        if i in range(l, r+1):
            return True
    return False


def task1(items: list[int], ranges: list[list[int]]):
    return [is_fresh(i, ranges) for i in items].count(True)


def task2(ranges: list[list[int]]) -> int:
    rs = sorted(ranges)
    start, end = rs[0]
    count = 0
    for r in rs[1:]:
        if r[0] > end:
            count += end - start + 1
            start, end = r
        elif r[1] > end:
            end = r[1]
    return count + end - start + 1


def main():
    with open("input.txt") as f:
        lines = [l.strip() for l in f.readlines()]
        sep = lines.index('')
        ranges = [[int(i) for i in l.split('-')] for l in lines[:sep]]
        items = [int(i) for i in lines[sep+1:]]
        print(task1(items, ranges))
        print(task2(ranges))


if __name__ == "__main__":
    main()
