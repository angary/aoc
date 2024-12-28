import re

from dataclasses import dataclass
from functools import cache
from itertools import islice

@dataclass(frozen=True, eq=True)
class Point:
    x: int
    y: int
    def __add__(self, other: "Point") -> "Point":
        return Point(self.x + other.x, self.y + other.y)


@dataclass
class Machine:
    a: Point
    b: Point
    target: Point


def get_point(line: str) -> Point:
    res = re.findall(r"(\d+)", line)
    return Point(int(res[0]), int(res[1]))


def get_machines(lines: list[str]) -> list[Machine]:
    machines = []
    for i in range(0, len(lines), 4):
        group = list(islice(lines, i, i + 4))
        machines.append(Machine(*map(get_point, group[:3])))
    return machines


def overshot(machine: Machine, curr: Point) -> bool:
    return curr.x > machine.target.x or curr.y > machine.target.y


def play1(machine: Machine) -> float:
    min_cost = float('inf')
    @cache
    def recurse(curr: Point, cost: int, a: int, b: int):
        nonlocal min_cost
        if overshot(machine, curr) or cost > min_cost or a > 100 or b > 100:
            return float('inf')
        if curr == machine.target:
            min_cost = min(cost, min_cost)
        recurse(curr + machine.a, cost + 3, a + 1, b)
        recurse(curr + machine.b, cost + 1, a, b + 1)
    recurse(Point(0, 0), 0, 0, 0)
    return min_cost


def play2(machine: Machine) -> float:
    a_x = machine.a.x * machine.b.y
    target_x = machine.target.x * machine.b.y
    a_y = machine.a.y * machine.b.x
    target_y = machine.target.y * machine.b.x
    n_a = (target_x - target_y) / (a_x - a_y)
    n_b = (machine.target.x - n_a * machine.a.x) / machine.b.x
    if not n_b.is_integer() or not n_a.is_integer():
        return float('inf')
    return n_a * 3 + n_b


def task1(machines: list[Machine]) -> int:
    costs = [play1(machine) for machine in machines]
    return sum(cost for cost in costs if cost != float('inf'))


def task2(machines: list[Machine]) -> int:
    shift = Point(10000000000000, 10000000000000)
    costs = [play2(Machine(m.a, m.b, m.target + shift)) for m in machines]
    return sum(cost for cost in costs if cost != float('inf'))


def main():
    with open("input.txt") as f:
        lines = f.readlines()
        machines = get_machines(lines)
        print(task1(machines))
        print(task2(machines))


if __name__ == "__main__":
    main()
