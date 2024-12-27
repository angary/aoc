from functools import cache


def blink(stones: list[str]) -> list[str]:
    res = []
    for stone in stones:
        if stone == "0":
            res.append("1")
        elif len(stone) % 2 == 0:
            n = len(stone)
            l, r = stone[:n//2], str(int(stone[n//2:]))
            res.extend([l, r])
        else:
            res.append(str(2024 * int(stone)))
    return res


@cache
def blink_optimised(stone: str, iterations: int) -> int:
    if iterations == 0:
        return 1
    if stone == "0":
        return blink_optimised("1", iterations - 1)
    elif len(stone) % 2 == 0:
        n = len(stone)
        l, r = stone[:n//2], str(int(stone[n//2:]))
        return blink_optimised(l, iterations - 1) + blink_optimised(r, iterations - 1)
    else:
        return blink_optimised(str(2024 * int(stone)), iterations - 1)
    

def task1(stones: list[str]) -> int:
    for _ in range(25):
        stones = blink(stones)
    return len(stones)


def task2(stones: list[str]) -> int:
    return sum(blink_optimised(stone, 75) for stone in stones)


def main():
    with open("input.txt") as f:
        stones = f.readline().split()
        print(task1(stones))
        print(task2(stones))


if __name__ == "__main__":
    main()