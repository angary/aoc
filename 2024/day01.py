def task1(xs: list[int], ys: list[int]) -> int:
    return sum(abs(x - y) for (x, y) in zip(xs, ys))


def task2(xs: list[int], ys: list[int]) -> int:
    freq = {y: ys.count(y) for y in ys}
    return sum(x * freq.get(x, 0) for x in xs)


def main():
    with open("input.txt") as f:
        lines = [[int(n) for n in line.split()] for line in f]
        xs, ys = map(sorted, zip(*lines))
        
        print(task1(xs, ys))
        print(task2(xs, ys))


if __name__ == "__main__":
    main()