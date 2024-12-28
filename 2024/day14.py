import math
import re

from dataclasses import dataclass

HEIGHT = 103
WIDTH = 101


@dataclass
class Robot:
    p: tuple[int, int]
    v: tuple[int, int]

    def move(self, iterations: int) -> "Robot":
        y = (self.p[0] + self.v[0] * iterations) % HEIGHT
        x = (self.p[1] + self.v[1] * iterations) % WIDTH
        return Robot((y, x), self.v)
    
    
    def get_quad(self) -> int:
        if self.p[0] < HEIGHT // 2 and self.p[1] < WIDTH // 2:
            return 0
        elif self.p[0] < HEIGHT // 2 and self.p[1] > WIDTH / 2:
            return 1
        elif self.p[0] > HEIGHT / 2 and self.p[1] < WIDTH // 2:
            return 2
        elif self.p[0] > HEIGHT / 2 and self.p[1] > WIDTH / 2:
            return 3
        return -1


def parse_line(line: str) -> Robot:
    n = [int(n) for n in re.findall(r"-?\b\d+\b", line)]
    return Robot((n[1], n[0]), (n[3], n[2]))


def get_safety_factor(robots: list[Robot]) -> int:
    quads = [r.get_quad() for r in robots]
    return math.prod(quads.count(i) for i in range(4))

def task1(robots: list[Robot]) -> int:
    return get_safety_factor([r.move(100) for r in robots])


def task2(robots: list[Robot]) -> int:
    test_robots = [r for r in robots]
    safety_factors = []
    for _ in range(10_000):
        safety_factors.append(get_safety_factor(test_robots))
        test_robots = [r.move(1) for r in test_robots]
    iterations = safety_factors.index(min(safety_factors))
    robot_locations = set(r.move(iterations).p for r in robots)
    for i in range(HEIGHT):
        for j in range(WIDTH):
            c = "#" if (i, j) in robot_locations else " "
            print(c, end="")
        print()
    print()
    return iterations


def main():
    with open("input.txt") as f:
        robots = [parse_line(l) for l in f.readlines()]
        print(task1(robots))
        print(task2(robots))


if __name__ == "__main__":
    main()
