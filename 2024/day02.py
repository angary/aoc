def dampen(lvl: list[int]) -> list[list[int]]:
    return [lvl[:i] + lvl[i+1:] for i in range(len(lvl))]


def is_valid(lvl: list[int]) -> bool:
    sorted_lvl = sorted(lvl)
    return (
        len(set(lvl)) == len(lvl)
        and all(sorted_lvl[i+1] - sorted_lvl[i] < 4 for i in range(len(lvl)-1))
        and (sorted_lvl == lvl or sorted_lvl[::-1] == lvl)
    )


def task1(lvls: list[int]) -> int:
    return sum(is_valid(lvl) for lvl in lvls)


def task2(lvls: list[int]) -> int:
    return sum(any(is_valid(dl) for dl in dampen(lvl)) for lvl in lvls)


def main():
    with open("input.txt") as f:
        lvls = [[int(n) for n in line.split()] for line in f]
        print(task1(lvls))
        print(task2(lvls))


if __name__ == "__main__":
    main()