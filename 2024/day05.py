import functools

def build_order_map(ordering: list[list[int]]) -> dict[int, set[int]]:
    order_map = {}
    for (v, k) in ordering:
        order_map[k] = order_map.get(k, set).union({v})
    return order_map
    

def is_valid(update: list[int], order_map: dict[int, set[int]]) -> bool:
    for i, x in enumerate(update):
        if not all(y not in order_map.get(x, {}) for y in update[i:]):
            return False
    return True
            

def task1(ordering: list[list[int]], updates: list[list[int]]) -> int:
    order_map = build_order_map(ordering)
    valid_updates = [update for update in updates if is_valid(update, order_map)]
    return sum(v[len(v) // 2] for v in valid_updates)


def task2(ordering: list[list[int]], updates:list[list[int]]) -> int:
    order_map = build_order_map(ordering)
    invalid_updates = [update for update in updates if not is_valid(update, order_map)]
    def cmp(x: int, y: int):
        if y in order_map.get(x):
            return -1
        elif x in order_map.get(y):
            return 1
        return 0
    sorted_updates = [sorted(u, key=functools.cmp_to_key(cmp)) for u in invalid_updates]
    return sum(v[len(v) // 2] for v in sorted_updates)


def main():
    with open("input.txt") as f:
        lines = [l.strip() for l in f.readlines()]
        ordering = [[int(n) for n in l.split("|")] for l in lines if "|" in l]
        updates = [[int(n) for n in l.split(",")] for l in lines if "," in l]
        print(task1(ordering, updates))
        print(task2(ordering, updates))


if __name__ == "__main__":
    main()
