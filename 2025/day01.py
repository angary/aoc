def task1(moves: list[int]) -> int:
    pos = 50
    counter = 0
    for move in moves:
        pos = (pos + move) % 100
        if pos == 0:
            counter += 1
    return counter


def task2(moves: list[int]) -> int:
    pos = 50
    counter = 0
    for move in moves:
        next_pos = pos + move
        if next_pos >= 100:
            counter += next_pos // 100
        elif next_pos < 0:
            counter += abs((next_pos - 1) // 100) # - 1 so that 0 gets counted
            if pos == 0: # We did not transition
                counter -= 1 
        pos = next_pos % 100
    return counter


def main():
    with open("input.txt") as f:
        lines = [l.strip() for l in f.readlines()]
        moves = [int(l[1:]) if l[0] == "R" else -int(l[1:]) for l in lines]
        print(task1(moves))
        print(task2(moves))


if __name__ == "__main__":
    main()
