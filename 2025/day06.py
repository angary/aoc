from math import prod


def apply(nums: list[int], f: str) -> int:
    return prod(nums) if f == "*" else sum(nums)


def task1(xss: list[str], operations: str) -> int:
    xss = [[int(x) for x in xs.split()] for xs in xss]
    operations = operations.split()
    count = 0
    for i, operation in enumerate(operations):
        nums = [xs[i] for xs in xss]
        count += apply(nums, operation)
    return count


def task2(xss: list[str], operations: str):
    indexes = [i for i, c in enumerate(operations) if c in "+*"] + [len(xss[0])]
    operations = operations.split()
    count = 0

    for i, operation in enumerate(operations):
        l, r = indexes[i], indexes[i+1]
        nums = [xs[l:r-1] for xs in xss]

        nums_t = []
        for j in range(len(nums[0]) - 1, -1, -1):
            nums_t.append(int("".join(num[j] for num in nums)))

        count += apply(nums_t, operation)
    return count


def main():
    with open("input.txt") as f:
        lines = [l for l in f.readlines()]
        print(task1(lines[:-1], lines[-1]))
        print(task2(lines[:-1], lines[-1]))


if __name__ == "__main__":
    main()
