dys = [-1, -1, -1, 0, 1, 1, 1, 0]
dxs = [-1, 0, 1, 1, 1, 0, -1, -1]


def neighhours(lines: list[str], i: int, j: int, height: int, width: int) -> list[str]:
    pos = [(i+dy, j+dx) for (dy, dx) in zip(dys, dxs)]
    return [lines[i][j] for (i, j) in pos if 0 <= i < height and 0 <= j < width]


def task1(lines: list[str]) -> int:
    height, width = len(lines), len(lines[0])
    total = 0
    for i in range(height):
        for j in range(width):
            if lines[i][j] == "@":
                if neighhours(lines, i, j, height, width).count("@") < 4:
                    total += 1
    return total
                

def task2(lines: list[str]) -> int:
    height, width = len(lines), len(lines[0])
    total = 0
    remove_positions = []
    for i in range(height):
        for j in range(width):
            if lines[i][j] == "@":
                if neighhours(lines, i, j, height, width).count("@") < 4:
                    total += 1
                    remove_positions.append((i, j))
    for (i, j) in remove_positions:
        lines[i] = lines[i][:j] + 'x' + lines[i][j+1:]
    if total == 0:
        return 0
    return total + task2(lines)


def main():
    with open("input.txt") as f:
        lines = [l.strip() for l in f.readlines()]
        print(task1(lines))
        print(task2(lines))


if __name__ == "__main__":
    main()